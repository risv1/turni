use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Error {
    code: i32,
    message: &'static str,
    display_message: &'static str,
}

pub const ERR_TENANT_NOT_FOUND: Error = Error {
    code: 1001,
    message: "Tenant not found",
    display_message: "Tenant not found",
};
pub const ERR_USER_NOT_FOUND: Error = Error {
    code: 1002,
    message: "User not found",
    display_message: "User not found",
};
pub const ERR_ROLE_NOT_FOUND: Error = Error {
    code: 1003,
    message: "Role not found",
    display_message: "Role not found",
};
pub const ERR_BAD_REQUEST: Error = Error {
    code: 400,
    message: "Bad Request",
    display_message: "Bad Request",
};
pub const ERR_UNAUTHORIZED: Error = Error {
    code: 401,
    message: "Unauthorized",
    display_message: "Unauthorized",
};
pub const ERR_FORBIDDEN: Error = Error {
    code: 403,
    message: "Forbidden",
    display_message: "Forbidden",
};
pub const ERR_NOT_FOUND: Error = Error {
    code: 404,
    message: "Not Found",
    display_message: "Not Found",
};
pub const ERR_INTERNAL_SERVER_ERROR: Error = Error {
    code: 500,
    message: "Internal Server Error",
    display_message: "Internal Server Error",
};
pub const ERR_SERVICE_UNAVAILABLE: Error = Error {
    code: 503,
    message: "Service Unavailable",
    display_message: "Service Unavailable",
};
