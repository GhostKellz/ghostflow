use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleSheetsNode;

#[async_trait]
impl Node for GoogleSheetsNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "google_sheets".to_string(),
            display_name: "Google Sheets".to_string(),
            description: "Read from and write to Google Sheets".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "access_token".to_string(),
                    display_name: "Access Token".to_string(),
                    description: "Google OAuth2 access token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "spreadsheet_id".to_string(),
                    display_name: "Spreadsheet ID".to_string(),
                    description: "Google Sheets spreadsheet ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Sheets operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("read".to_string())),
                },
                NodeParameter {
                    name: "range".to_string(),
                    display_name: "Range".to_string(),
                    description: "Cell range (e.g., A1:D10, Sheet1!A:Z)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(Value::String("A:Z".to_string())),
                },
                NodeParameter {
                    name: "sheet_name".to_string(),
                    display_name: "Sheet Name".to_string(),
                    description: "Name of the sheet/tab".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(Value::String("Sheet1".to_string())),
                },
                NodeParameter {
                    name: "values".to_string(),
                    display_name: "Values".to_string(),
                    description: "Data to write (JSON array of arrays)".to_string(),
                    parameter_type: ParameterType::Json,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "value_input_option".to_string(),
                    display_name: "Value Input Option".to_string(),
                    description: "How input data should be interpreted".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("USER_ENTERED".to_string())),
                },
                NodeParameter {
                    name: "include_headers".to_string(),
                    display_name: "Include Headers".to_string(),
                    description: "Include first row as headers in output".to_string(),
                    parameter_type: ParameterType::Boolean,
                    required: false,
                    default_value: Some(Value::Bool(true)),
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "data".to_string(), "headers".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let access_token = context.get_parameter("access_token")
            .and_then(|v| v.as_string())
            .ok_or("Access token is required")?;
        
        let spreadsheet_id = context.get_parameter("spreadsheet_id")
            .and_then(|v| v.as_string())
            .ok_or("Spreadsheet ID is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("read".to_string());
        
        let sheet_name = context.get_parameter("sheet_name")
            .and_then(|v| v.as_string())
            .unwrap_or("Sheet1".to_string());
        
        let range = context.get_parameter("range")
            .and_then(|v| v.as_string())
            .unwrap_or("A:Z".to_string());

        let client = reqwest::Client::new();
        let base_url = "https://sheets.googleapis.com/v4/spreadsheets";

        let full_range = if range.contains('!') {
            range
        } else {
            format!("{}!{}", sheet_name, range)
        };

        let result = match operation.as_str() {
            "read" => {
                let encoded_range = urlencoding::encode(&full_range);
                let response = client
                    .get(&format!("{}/{}/values/{}", base_url, spreadsheet_id, encoded_range))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                
                let include_headers = context.get_parameter("include_headers")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);

                let values = data["values"].as_array().unwrap_or(&vec![]);
                
                if include_headers && !values.is_empty() {
                    let headers = &values[0];
                    let rows = &values[1..];
                    
                    json!({
                        "values": values,
                        "headers": headers,
                        "data": rows,
                        "range": data["range"],
                        "majorDimension": data["majorDimension"]
                    })
                } else {
                    json!({
                        "values": values,
                        "data": values,
                        "range": data["range"],
                        "majorDimension": data["majorDimension"]
                    })
                }
            },
            "write" => {
                let values = context.get_parameter("values")
                    .ok_or("Values are required for write operation")?;
                
                let value_input_option = context.get_parameter("value_input_option")
                    .and_then(|v| v.as_string())
                    .unwrap_or("USER_ENTERED".to_string());

                let encoded_range = urlencoding::encode(&full_range);
                let response = client
                    .put(&format!("{}/{}/values/{}", base_url, spreadsheet_id, encoded_range))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("valueInputOption", &value_input_option)])
                    .json(&json!({
                        "values": values
                    }))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "append" => {
                let values = context.get_parameter("values")
                    .ok_or("Values are required for append operation")?;
                
                let value_input_option = context.get_parameter("value_input_option")
                    .and_then(|v| v.as_string())
                    .unwrap_or("USER_ENTERED".to_string());

                let encoded_range = urlencoding::encode(&full_range);
                let response = client
                    .post(&format!("{}/{}/values/{}:append", base_url, spreadsheet_id, encoded_range))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[
                        ("valueInputOption", value_input_option.as_str()),
                        ("insertDataOption", "INSERT_ROWS")
                    ])
                    .json(&json!({
                        "values": values
                    }))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "clear" => {
                let encoded_range = urlencoding::encode(&full_range);
                let response = client
                    .post(&format!("{}/{}/values/{}:clear", base_url, spreadsheet_id, encoded_range))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get_info" => {
                let response = client
                    .get(&format!("{}/{}", base_url, spreadsheet_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "batch_get" => {
                let ranges = context.get_parameter("ranges")
                    .and_then(|v| v.as_array())
                    .ok_or("Ranges array is required for batch_get operation")?;

                let range_strings: Vec<String> = ranges.iter()
                    .filter_map(|r| r.as_string())
                    .map(|r| if r.contains('!') { r } else { format!("{}!{}", sheet_name, r) })
                    .collect();

                let response = client
                    .get(&format!("{}/{}/values:batchGet", base_url, spreadsheet_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&range_strings.iter().map(|r| ("ranges", r.as_str())).collect::<Vec<_>>())
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            _ => {
                return Err(format!("Unknown operation: {}", operation).into());
            }
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result.clone()));
        
        // Extract specific data for convenience
        if let Some(values) = result.get("values").or(result.get("data")) {
            outputs.insert("data".to_string(), values.clone().into());
        }
        
        if let Some(headers) = result.get("headers") {
            outputs.insert("headers".to_string(), headers.clone().into());
        }
        
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleSheetsFormulaNode;

#[async_trait]
impl Node for GoogleSheetsFormulaNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "google_sheets_formula".to_string(),
            display_name: "Google Sheets Formula".to_string(),
            description: "Execute formulas and advanced operations in Google Sheets".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "access_token".to_string(),
                    display_name: "Access Token".to_string(),
                    description: "Google OAuth2 access token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "spreadsheet_id".to_string(),
                    display_name: "Spreadsheet ID".to_string(),
                    description: "Google Sheets spreadsheet ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Formula operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("write_formula".to_string())),
                },
                NodeParameter {
                    name: "range".to_string(),
                    display_name: "Range".to_string(),
                    description: "Cell range for formula".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "formula".to_string(),
                    display_name: "Formula".to_string(),
                    description: "Excel/Google Sheets formula (e.g., =SUM(A1:A10))".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "sheet_name".to_string(),
                    display_name: "Sheet Name".to_string(),
                    description: "Name of the sheet/tab".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(Value::String("Sheet1".to_string())),
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "calculated_value".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let access_token = context.get_parameter("access_token")
            .and_then(|v| v.as_string())
            .ok_or("Access token is required")?;
        
        let spreadsheet_id = context.get_parameter("spreadsheet_id")
            .and_then(|v| v.as_string())
            .ok_or("Spreadsheet ID is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("write_formula".to_string());
        
        let sheet_name = context.get_parameter("sheet_name")
            .and_then(|v| v.as_string())
            .unwrap_or("Sheet1".to_string());
        
        let range = context.get_parameter("range")
            .and_then(|v| v.as_string())
            .ok_or("Range is required")?;

        let client = reqwest::Client::new();
        let base_url = "https://sheets.googleapis.com/v4/spreadsheets";

        let full_range = if range.contains('!') {
            range
        } else {
            format!("{}!{}", sheet_name, range)
        };

        let result = match operation.as_str() {
            "write_formula" => {
                let formula = context.get_parameter("formula")
                    .and_then(|v| v.as_string())
                    .ok_or("Formula is required for write_formula operation")?;

                let encoded_range = urlencoding::encode(&full_range);
                let response = client
                    .put(&format!("{}/{}/values/{}", base_url, spreadsheet_id, encoded_range))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("valueInputOption", "USER_ENTERED")])
                    .json(&json!({
                        "values": [[formula]]
                    }))
                    .send()
                    .await?;

                let write_result: serde_json::Value = response.json().await?;

                // Read back the calculated value
                let read_response = client
                    .get(&format!("{}/{}/values/{}", base_url, spreadsheet_id, encoded_range))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("valueRenderOption", "FORMATTED_VALUE")])
                    .send()
                    .await?;

                let read_result: serde_json::Value = read_response.json().await?;
                
                json!({
                    "write_result": write_result,
                    "calculated_result": read_result,
                    "formula": formula,
                    "range": full_range
                })
            },
            "batch_formula" => {
                let formulas = context.get_parameter("formulas")
                    .and_then(|v| v.as_object())
                    .ok_or("Formulas object is required for batch_formula operation")?;

                let mut batch_data = Vec::new();
                
                for (range, formula) in formulas.iter() {
                    let full_range = if range.contains('!') {
                        range.clone()
                    } else {
                        format!("{}!{}", sheet_name, range)
                    };
                    
                    batch_data.push(json!({
                        "range": full_range,
                        "values": [[formula.as_str().unwrap_or("")]]
                    }));
                }

                let response = client
                    .post(&format!("{}/{}/values:batchUpdate", base_url, spreadsheet_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .json(&json!({
                        "valueInputOption": "USER_ENTERED",
                        "data": batch_data
                    }))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            _ => {
                return Err(format!("Unknown operation: {}", operation).into());
            }
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result.clone()));
        
        // Extract calculated value if available
        if let Some(calc_result) = result.get("calculated_result") {
            if let Some(values) = calc_result.get("values").and_then(|v| v.as_array()) {
                if let Some(first_row) = values.first().and_then(|r| r.as_array()) {
                    if let Some(first_cell) = first_row.first() {
                        outputs.insert("calculated_value".to_string(), first_cell.clone().into());
                    }
                }
            }
        }
        
        Ok(outputs)
    }
}