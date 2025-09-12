use axum::{
    async_trait,
    extract::{FromRequestParts, Query, State},
    http::{request::Parts, StatusCode},
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::{AppState, ApiError, ApiResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub workspace_id: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub workspace_id: String,
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
    pub iss: String,       // Issuer
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub workspace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: Option<UserRole>,
    pub workspace_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn generate_token(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            sub: user.id.clone(),
            email: user.email.clone(),
            name: user.name.clone(),
            role: user.role.clone(),
            workspace_id: user.workspace_id.clone(),
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            iss: "ghostflow".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
    }

    pub fn generate_refresh_token(&self, user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            sub: user_id.to_string(),
            email: String::new(),
            name: String::new(),
            role: UserRole::User,
            workspace_id: String::new(),
            exp: (Utc::now() + Duration::days(30)).timestamp(),
            iat: Utc::now().timestamp(),
            iss: "ghostflow-refresh".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&["ghostflow"]);
        
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        ).map(|data| data.claims)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&["ghostflow-refresh"]);
        
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        ).map(|data| data.claims)
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<User, String> {
        // TODO: Implement actual password verification with database
        // For now, return mock user for demo purposes
        
        if email == "admin@ghostflow.dev" && password == "admin123" {
            Ok(User {
                id: "user_001".to_string(),
                email: email.to_string(),
                name: "Admin User".to_string(),
                role: UserRole::Admin,
                workspace_id: "workspace_001".to_string(),
                created_at: Utc::now() - Duration::days(30),
                last_login: Some(Utc::now()),
                is_active: true,
            })
        } else if email == "user@ghostflow.dev" && password == "user123" {
            Ok(User {
                id: "user_002".to_string(),
                email: email.to_string(),
                name: "Regular User".to_string(),
                role: UserRole::User,
                workspace_id: "workspace_001".to_string(),
                created_at: Utc::now() - Duration::days(15),
                last_login: Some(Utc::now()),
                is_active: true,
            })
        } else {
            Err("Invalid credentials".to_string())
        }
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, String> {
        // TODO: Implement actual database lookup
        match user_id {
            "user_001" => Ok(User {
                id: "user_001".to_string(),
                email: "admin@ghostflow.dev".to_string(),
                name: "Admin User".to_string(),
                role: UserRole::Admin,
                workspace_id: "workspace_001".to_string(),
                created_at: Utc::now() - Duration::days(30),
                last_login: Some(Utc::now()),
                is_active: true,
            }),
            "user_002" => Ok(User {
                id: "user_002".to_string(),
                email: "user@ghostflow.dev".to_string(),
                name: "Regular User".to_string(),
                role: UserRole::User,
                workspace_id: "workspace_001".to_string(),
                created_at: Utc::now() - Duration::days(15),
                last_login: Some(Utc::now()),
                is_active: true,
            }),
            _ => Err("User not found".to_string()),
        }
    }
}

// Authentication middleware
pub struct AuthenticatedUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    Arc<AppState>: FromRequestParts<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let State(app_state): State<Arc<AppState>> = 
            State::from_request_parts(parts, state).await.map_err(|_| AuthError::ServerError)?;

        // Extract token from Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(&header[7..])
                } else {
                    None
                }
            })
            .ok_or(AuthError::MissingToken)?;

        // Verify token
        let auth_service = AuthService::new("your-secret-key".to_string()); // TODO: Get from config
        let claims = auth_service.verify_token(auth_header)
            .map_err(|_| AuthError::InvalidToken)?;

        // Get user from database
        let user = auth_service.get_user_by_id(&claims.sub).await
            .map_err(|_| AuthError::UserNotFound)?;

        if !user.is_active {
            return Err(AuthError::UserInactive);
        }

        Ok(AuthenticatedUser(user))
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    UserNotFound,
    UserInactive,
    ServerError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User not found"),
            AuthError::UserInactive => (StatusCode::FORBIDDEN, "User account is inactive"),
            AuthError::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = serde_json::json!({
            "error": message,
            "status": status.as_u16()
        });

        (status, Json(body)).into_response()
    }
}

// Auth route handlers
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let auth_service = AuthService::new("your-secret-key".to_string()); // TODO: Get from config

    // Authenticate user
    let user = auth_service.authenticate_user(&request.email, &request.password).await
        .map_err(|e| ApiError::Unauthorized(e))?;

    // Generate tokens
    let token = auth_service.generate_token(&user)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to generate token: {}", e)))?;
    
    let refresh_token = auth_service.generate_refresh_token(&user.id)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to generate refresh token: {}", e)))?;

    // TODO: Store refresh token in database
    // TODO: Update last_login timestamp

    let response = LoginResponse {
        token,
        refresh_token,
        expires_in: 86400, // 24 hours
        user: UserInfo {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            workspace_id: user.workspace_id,
        },
    };

    Ok(Json(response))
}

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshTokenRequest>,
) -> ApiResult<Json<RefreshTokenResponse>> {
    let auth_service = AuthService::new("your-secret-key".to_string()); // TODO: Get from config

    // Verify refresh token
    let claims = auth_service.verify_refresh_token(&request.refresh_token)
        .map_err(|_| ApiError::Unauthorized("Invalid refresh token".to_string()))?;

    // Get user
    let user = auth_service.get_user_by_id(&claims.sub).await
        .map_err(|_| ApiError::Unauthorized("User not found".to_string()))?;

    if !user.is_active {
        return Err(ApiError::Unauthorized("User account is inactive".to_string()));
    }

    // Generate new access token
    let new_token = auth_service.generate_token(&user)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to generate token: {}", e)))?;

    let response = RefreshTokenResponse {
        token: new_token,
        expires_in: 86400, // 24 hours
    };

    Ok(Json(response))
}

pub async fn get_current_user(
    auth_user: AuthenticatedUser,
) -> ApiResult<Json<UserInfo>> {
    let user_info = UserInfo {
        id: auth_user.0.id,
        email: auth_user.0.email,
        name: auth_user.0.name,
        role: auth_user.0.role,
        workspace_id: auth_user.0.workspace_id,
    };

    Ok(Json(user_info))
}

pub async fn create_user(
    auth_user: AuthenticatedUser,
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> ApiResult<Json<CreateUserResponse>> {
    // Check if current user has admin privileges
    if auth_user.0.role != UserRole::Admin {
        return Err(ApiError::Forbidden("Admin privileges required".to_string()));
    }

    // TODO: Validate email format
    // TODO: Check if user already exists
    // TODO: Hash password
    // TODO: Store in database

    let new_user = UserInfo {
        id: format!("user_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
        email: request.email,
        name: request.name,
        role: request.role.unwrap_or(UserRole::User),
        workspace_id: request.workspace_id.unwrap_or_else(|| auth_user.0.workspace_id.clone()),
    };

    let response = CreateUserResponse {
        user: new_user,
    };

    Ok(Json(response))
}

pub async fn change_password(
    auth_user: AuthenticatedUser,
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChangePasswordRequest>,
) -> ApiResult<StatusCode> {
    // TODO: Verify current password
    // TODO: Hash new password
    // TODO: Update in database
    // TODO: Invalidate all existing tokens

    Ok(StatusCode::NO_CONTENT)
}

pub async fn logout(
    auth_user: AuthenticatedUser,
    State(state): State<Arc<AppState>>,
) -> ApiResult<StatusCode> {
    // TODO: Invalidate tokens in database/redis
    // TODO: Add token to blacklist

    Ok(StatusCode::NO_CONTENT)
}