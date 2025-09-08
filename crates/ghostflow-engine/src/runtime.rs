use crate::{FlowExecutor, FlowScheduler};
use ghostflow_core::{GhostFlowError, NodeRegistry, Result};
use ghostflow_schema::{ExecutionTrigger, Flow, FlowExecution};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct FlowRuntime {
    executor: FlowExecutor,
    scheduler: FlowScheduler,
    flows: Arc<RwLock<HashMap<Uuid, Flow>>>,
    node_registry: Arc<dyn NodeRegistry>,
    running: Arc<RwLock<bool>>,
}

impl FlowRuntime {
    pub fn new(node_registry: Arc<dyn NodeRegistry>) -> Self {
        let executor = FlowExecutor::new(node_registry.clone());
        let scheduler = FlowScheduler::new();
        
        Self {
            executor,
            scheduler,
            flows: Arc::new(RwLock::new(HashMap::new())),
            node_registry,
            running: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if *running {
            return Err(GhostFlowError::InternalError {
                message: "Runtime is already running".to_string(),
            });
        }
        
        *running = true;
        drop(running);
        
        info!("Starting GhostFlow runtime");
        
        // Start the scheduler loop
        let scheduler = self.scheduler.clone();
        let executor = self.executor.clone();
        let running_clone = self.running.clone();
        
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(10)); // Check every 10 seconds
            
            loop {
                ticker.tick().await;
                
                // Check if runtime is still running
                {
                    let running = running_clone.read().await;
                    if !*running {
                        info!("Scheduler loop stopping");
                        break;
                    }
                }
                
                // Get flows that are ready to run
                let ready_flows = scheduler.get_ready_flows().await;
                
                for (flow, trigger) in ready_flows {
                    info!("Executing scheduled flow {} triggered by {}", flow.id, trigger.id);
                    
                    let execution_trigger = ExecutionTrigger {
                        trigger_type: match trigger.trigger_type {
                            ghostflow_schema::TriggerType::Cron { .. } => "cron".to_string(),
                            ghostflow_schema::TriggerType::Webhook { .. } => "webhook".to_string(),
                            ghostflow_schema::TriggerType::Manual => "manual".to_string(),
                        },
                        source: Some(trigger.id.clone()),
                        metadata: HashMap::new(),
                    };
                    
                    // Execute the flow
                    match executor.execute_flow(&flow, serde_json::Value::Null, execution_trigger).await {
                        Ok(execution) => {
                            info!("Flow execution {} completed with status {:?}", execution.id, execution.status);
                            
                            // Update trigger next run time for cron triggers
                            if let Err(e) = scheduler.update_trigger_next_run(&flow.id, &trigger.id).await {
                                error!("Failed to update trigger next run: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Flow execution failed: {}", e);
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = false;
        info!("Stopping GhostFlow runtime");
        Ok(())
    }

    pub async fn deploy_flow(&self, flow: Flow) -> Result<()> {
        info!("Deploying flow {}: {}", flow.id, flow.name);
        
        // Validate the flow
        self.validate_flow(&flow).await?;
        
        // Store the flow
        {
            let mut flows = self.flows.write().await;
            flows.insert(flow.id, flow.clone());
        }
        
        // Schedule the flow
        self.scheduler.schedule_flow(flow).await?;
        
        Ok(())
    }

    pub async fn undeploy_flow(&self, flow_id: &Uuid) -> Result<()> {
        info!("Undeploying flow {}", flow_id);
        
        // Remove from scheduler
        self.scheduler.unschedule_flow(flow_id).await?;
        
        // Remove from flows
        {
            let mut flows = self.flows.write().await;
            flows.remove(flow_id);
        }
        
        Ok(())
    }

    pub async fn execute_flow_manually(
        &self,
        flow_id: &Uuid,
        input_data: serde_json::Value,
    ) -> Result<FlowExecution> {
        let flow = {
            let flows = self.flows.read().await;
            flows.get(flow_id).cloned().ok_or_else(|| GhostFlowError::NotFoundError {
                resource_type: "flow".to_string(),
                id: flow_id.to_string(),
            })?
        };
        
        let execution_trigger = ExecutionTrigger {
            trigger_type: "manual".to_string(),
            source: None,
            metadata: HashMap::new(),
        };
        
        self.executor.execute_flow(&flow, input_data, execution_trigger).await
    }

    pub async fn list_flows(&self) -> Vec<Flow> {
        let flows = self.flows.read().await;
        flows.values().cloned().collect()
    }

    pub async fn get_flow(&self, flow_id: &Uuid) -> Option<Flow> {
        let flows = self.flows.read().await;
        flows.get(flow_id).cloned()
    }

    async fn validate_flow(&self, flow: &Flow) -> Result<()> {
        // Basic validation
        if flow.nodes.is_empty() {
            return Err(GhostFlowError::ValidationError {
                message: "Flow must contain at least one node".to_string(),
            });
        }

        // Validate all nodes exist in registry
        for (node_id, node) in &flow.nodes {
            if !self.node_registry.validate_node_type(&node.node_type) {
                return Err(GhostFlowError::ValidationError {
                    message: format!("Unknown node type '{}' in node '{}'", node.node_type, node_id),
                });
            }
        }

        // Validate edges reference existing nodes
        for edge in &flow.edges {
            if !flow.nodes.contains_key(&edge.source_node) {
                return Err(GhostFlowError::ValidationError {
                    message: format!("Edge references unknown source node '{}'", edge.source_node),
                });
            }
            
            if !flow.nodes.contains_key(&edge.target_node) {
                return Err(GhostFlowError::ValidationError {
                    message: format!("Edge references unknown target node '{}'", edge.target_node),
                });
            }
        }

        Ok(())
    }
}