use ghostflow_core::{GhostFlowError, Result};
use ghostflow_schema::{Flow, FlowTrigger, TriggerType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Clone)]
pub struct FlowScheduler {
    scheduled_flows: Arc<RwLock<HashMap<Uuid, ScheduledFlow>>>,
}

#[derive(Debug, Clone)]
struct ScheduledFlow {
    flow: Flow,
    triggers: Vec<ScheduledTrigger>,
}

#[derive(Debug, Clone)]
struct ScheduledTrigger {
    trigger: FlowTrigger,
    next_run: Option<chrono::DateTime<chrono::Utc>>,
}

impl FlowScheduler {
    pub fn new() -> Self {
        Self {
            scheduled_flows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn schedule_flow(&self, flow: Flow) -> Result<()> {
        let mut scheduled_flows = self.scheduled_flows.write().await;
        
        let mut scheduled_triggers = Vec::new();
        
        for trigger in &flow.triggers {
            if !trigger.enabled {
                continue;
            }
            
            let scheduled_trigger = match &trigger.trigger_type {
                TriggerType::Cron { expression, timezone } => {
                    let next_run = self.calculate_next_cron_run(expression, timezone.as_deref())?;
                    ScheduledTrigger {
                        trigger: trigger.clone(),
                        next_run: Some(next_run),
                    }
                }
                TriggerType::Webhook { .. } => {
                    // Webhooks don't have scheduled runs
                    ScheduledTrigger {
                        trigger: trigger.clone(),
                        next_run: None,
                    }
                }
                TriggerType::Manual => {
                    // Manual triggers don't have scheduled runs
                    ScheduledTrigger {
                        trigger: trigger.clone(),
                        next_run: None,
                    }
                }
            };
            
            scheduled_triggers.push(scheduled_trigger);
        }
        
        let scheduled_flow = ScheduledFlow {
            flow: flow.clone(),
            triggers: scheduled_triggers,
        };
        
        scheduled_flows.insert(flow.id, scheduled_flow);
        
        info!("Scheduled flow {} with {} triggers", flow.id, flow.triggers.len());
        
        Ok(())
    }

    pub async fn unschedule_flow(&self, flow_id: &Uuid) -> Result<()> {
        let mut scheduled_flows = self.scheduled_flows.write().await;
        
        if scheduled_flows.remove(flow_id).is_some() {
            info!("Unscheduled flow {}", flow_id);
            Ok(())
        } else {
            Err(GhostFlowError::NotFoundError {
                resource_type: "scheduled_flow".to_string(),
                id: flow_id.to_string(),
            })
        }
    }

    pub async fn get_ready_flows(&self) -> Vec<(Flow, FlowTrigger)> {
        let now = chrono::Utc::now();
        let mut ready_flows = Vec::new();
        
        let scheduled_flows = self.scheduled_flows.read().await;
        
        for scheduled_flow in scheduled_flows.values() {
            for scheduled_trigger in &scheduled_flow.triggers {
                if let Some(next_run) = scheduled_trigger.next_run {
                    if next_run <= now {
                        ready_flows.push((
                            scheduled_flow.flow.clone(),
                            scheduled_trigger.trigger.clone(),
                        ));
                    }
                }
            }
        }
        
        ready_flows
    }

    pub async fn update_trigger_next_run(&self, flow_id: &Uuid, trigger_id: &str) -> Result<()> {
        let mut scheduled_flows = self.scheduled_flows.write().await;
        
        if let Some(scheduled_flow) = scheduled_flows.get_mut(flow_id) {
            for scheduled_trigger in &mut scheduled_flow.triggers {
                if scheduled_trigger.trigger.id == trigger_id {
                    match &scheduled_trigger.trigger.trigger_type {
                        TriggerType::Cron { expression, timezone } => {
                            let next_run = self.calculate_next_cron_run(expression, timezone.as_deref())?;
                            scheduled_trigger.next_run = Some(next_run);
                            info!("Updated next run for trigger {} to {}", trigger_id, next_run);
                        }
                        _ => {
                            // Non-cron triggers don't need next run updates
                        }
                    }
                    break;
                }
            }
        }
        
        Ok(())
    }

    fn calculate_next_cron_run(
        &self,
        expression: &str,
        _timezone: Option<&str>,
    ) -> Result<chrono::DateTime<chrono::Utc>> {
        // Simple implementation - in a real system, use a proper cron library like `cron`
        // For now, we'll just add 1 minute to simulate a basic schedule
        
        // TODO: Implement proper cron parsing
        // - Support standard cron expressions (minute hour day month weekday)
        // - Handle timezone conversions
        // - Validate expressions
        
        let next_run = chrono::Utc::now() + chrono::Duration::minutes(1);
        
        Ok(next_run)
    }

    pub async fn list_scheduled_flows(&self) -> Vec<Flow> {
        let scheduled_flows = self.scheduled_flows.read().await;
        scheduled_flows
            .values()
            .map(|sf| sf.flow.clone())
            .collect()
    }
}

impl Default for FlowScheduler {
    fn default() -> Self {
        Self::new()
    }
}