use async_trait::async_trait;
use futures::future::join_all;
use ghostflow_core::{GhostFlowError, Node, NodeRegistry, Result};
use ghostflow_schema::{
    ExecutionContext, ExecutionStatus, Flow, FlowExecution, NodeExecution, ExecutionTrigger,
    ExecutionMetadata, ExecutionError, ErrorType,
};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Clone)]
pub struct FlowExecutor {
    node_registry: Arc<dyn NodeRegistry>,
    max_concurrent_nodes: usize,
}

impl FlowExecutor {
    pub fn new(node_registry: Arc<dyn NodeRegistry>) -> Self {
        Self {
            node_registry,
            max_concurrent_nodes: 10,
        }
    }

    pub async fn execute_flow(
        &self,
        flow: &Flow,
        input_data: serde_json::Value,
        trigger: ExecutionTrigger,
    ) -> Result<FlowExecution> {
        let execution_id = Uuid::new_v4();
        let start_time = Instant::now();
        
        info!("Starting flow execution {} for flow {}", execution_id, flow.id);

        let mut execution = FlowExecution {
            id: execution_id,
            flow_id: flow.id,
            flow_version: flow.version.clone(),
            status: ExecutionStatus::Running,
            trigger,
            input_data: input_data.clone(),
            output_data: None,
            error: None,
            node_executions: HashMap::new(),
            started_at: chrono::Utc::now(),
            completed_at: None,
            execution_time_ms: None,
            metadata: ExecutionMetadata {
                executor_id: "default".to_string(),
                environment: "local".to_string(),
                correlation_id: None,
                trace_id: Some(execution_id.to_string()),
                span_id: None,
            },
        };

        match self.execute_flow_internal(flow, &input_data, &execution_id).await {
            Ok(result) => {
                execution.status = ExecutionStatus::Completed;
                execution.output_data = Some(result);
                execution.completed_at = Some(chrono::Utc::now());
                execution.execution_time_ms = Some(start_time.elapsed().as_millis() as u64);
                
                info!("Flow execution {} completed successfully", execution_id);
            }
            Err(error) => {
                execution.status = ExecutionStatus::Failed;
                execution.error = Some(ExecutionError {
                    error_type: ErrorType::InternalError,
                    message: error.to_string(),
                    details: None,
                    retryable: true,
                });
                execution.completed_at = Some(chrono::Utc::now());
                execution.execution_time_ms = Some(start_time.elapsed().as_millis() as u64);
                
                error!("Flow execution {} failed: {}", execution_id, error);
            }
        }

        Ok(execution)
    }

    async fn execute_flow_internal(
        &self,
        flow: &Flow,
        input_data: &serde_json::Value,
        execution_id: &Uuid,
    ) -> Result<serde_json::Value> {
        // Build execution graph
        let execution_order = self.build_execution_order(flow)?;
        let mut node_results: HashMap<String, serde_json::Value> = HashMap::new();
        let mut variables = HashMap::new();
        
        // Add input data to variables
        variables.insert("input".to_string(), input_data.clone());

        // Execute nodes in topological order
        for node_batch in execution_order {
            let node_ids: Vec<String> = node_batch.clone();
            let futures: Vec<_> = node_batch
                .into_iter()
                .map(|node_id| {
                    let flow_node = flow.nodes.get(&node_id).unwrap();
                    let context = ExecutionContext {
                        execution_id: *execution_id,
                        flow_id: flow.id,
                        node_id: node_id.clone(),
                        input: self.resolve_node_input(flow_node, &node_results, &variables),
                        variables: variables.clone(),
                        secrets: HashMap::new(), // TODO: integrate with secrets manager
                        artifacts: HashMap::new(),
                    };
                    
                    self.execute_node(flow_node.node_type.clone(), context)
                })
                .collect();

            // Execute nodes in parallel within the batch
            let batch_results = join_all(futures).await;
            
            for (i, result) in batch_results.into_iter().enumerate() {
                let node_id = &node_ids[i];
                match result {
                    Ok(output) => {
                        node_results.insert(node_id.clone(), output);
                    }
                    Err(error) => {
                        error!("Node {} failed: {}", node_id, error);
                        return Err(error);
                    }
                }
            }
        }

        // Determine final output
        let final_output = if let Some(last_node_id) = flow.nodes.keys().last() {
            node_results.get(last_node_id).cloned().unwrap_or(serde_json::Value::Null)
        } else {
            serde_json::Value::Null
        };

        Ok(final_output)
    }

    async fn execute_node(
        &self,
        node_type: String,
        context: ExecutionContext,
    ) -> Result<serde_json::Value> {
        let node = self.node_registry
            .get_node(&node_type)
            .ok_or_else(|| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: format!("Unknown node type: {}", node_type),
            })?;

        // Validate node inputs
        node.validate(&context).await?;

        // Execute the node
        let result = node.execute(context).await?;
        
        Ok(result)
    }

    fn resolve_node_input(
        &self,
        flow_node: &ghostflow_schema::FlowNode,
        node_results: &HashMap<String, serde_json::Value>,
        variables: &HashMap<String, serde_json::Value>,
    ) -> serde_json::Value {
        // Simple parameter resolution - in a real implementation, this would be more sophisticated
        let mut resolved_params = flow_node.parameters.clone();
        
        // TODO: Implement proper parameter interpolation
        // - Support for {{$node.output}} syntax
        // - Variable substitution
        // - Expression evaluation
        
        serde_json::Value::Object(
            resolved_params
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect()
        )
    }

    fn build_execution_order(&self, flow: &Flow) -> Result<Vec<Vec<String>>> {
        // Simple topological sort implementation
        // In a real implementation, this would handle cycles, conditional execution, etc.
        
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        
        // Initialize
        for node_id in flow.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
            adjacency.insert(node_id.clone(), Vec::new());
        }
        
        // Build graph
        for edge in &flow.edges {
            adjacency
                .get_mut(&edge.source_node)
                .unwrap()
                .push(edge.target_node.clone());
            
            *in_degree.get_mut(&edge.target_node).unwrap() += 1;
        }
        
        let mut result = Vec::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        
        // Find nodes with no dependencies
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id.clone());
            }
        }
        
        while !queue.is_empty() {
            let mut current_batch = Vec::new();
            let batch_size = queue.len();
            
            for _ in 0..batch_size {
                if let Some(node_id) = queue.pop_front() {
                    current_batch.push(node_id.clone());
                    
                    // Update dependencies
                    if let Some(neighbors) = adjacency.get(&node_id) {
                        for neighbor in neighbors {
                            if let Some(degree) = in_degree.get_mut(neighbor) {
                                *degree -= 1;
                                if *degree == 0 {
                                    queue.push_back(neighbor.clone());
                                }
                            }
                        }
                    }
                }
            }
            
            if !current_batch.is_empty() {
                result.push(current_batch);
            }
        }
        
        // Check for cycles
        if result.iter().map(|batch| batch.len()).sum::<usize>() != flow.nodes.len() {
            return Err(GhostFlowError::ValidationError {
                message: "Flow contains cycles".to_string(),
            });
        }
        
        Ok(result)
    }
}