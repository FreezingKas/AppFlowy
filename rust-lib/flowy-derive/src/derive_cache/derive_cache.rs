pub enum TypeCategory {
    Array,
    Map,
    Str,
    Protobuf,
    Bytes,
    Enum,
    Opt,
    Primitive,
}
// auto generate, do not edit
pub fn category_from_str(type_str: &str) -> TypeCategory {
    match type_str {
        "Vec" => TypeCategory::Array,
        "HashMap" => TypeCategory::Map,
        "u8" => TypeCategory::Bytes,
        "String" => TypeCategory::Str,
        "ObservableSubject"
        | "KeyValue"
        | "QueryAppRequest"
        | "QueryAppParams"
        | "CreateAppRequest"
        | "ColorStyle"
        | "CreateAppParams"
        | "App"
        | "RepeatedApp"
        | "UpdateAppRequest"
        | "UpdateAppParams"
        | "DeleteAppRequest"
        | "DeleteAppParams"
        | "UpdateWorkspaceRequest"
        | "UpdateWorkspaceParams"
        | "DeleteWorkspaceRequest"
        | "DeleteWorkspaceParams"
        | "CreateWorkspaceRequest"
        | "CreateWorkspaceParams"
        | "Workspace"
        | "Workspaces"
        | "QueryWorkspaceRequest"
        | "QueryWorkspaceParams"
        | "CurrentWorkspace"
        | "UpdateViewRequest"
        | "DeleteViewRequest"
        | "QueryViewRequest"
        | "CreateViewRequest"
        | "View"
        | "RepeatedView"
        | "WorkspaceError"
        | "CreateDocRequest"
        | "DocInfo"
        | "DocData"
        | "QueryDocRequest"
        | "QueryDocDataRequest"
        | "UpdateDocRequest"
        | "DocError"
        | "FFIRequest"
        | "FFIResponse"
        | "UserDetail"
        | "UpdateUserRequest"
        | "UpdateUserParams"
        | "SignUpRequest"
        | "SignUpParams"
        | "SignUpResponse"
        | "SignInRequest"
        | "SignInParams"
        | "SignInResponse"
        | "UserError"
        => TypeCategory::Protobuf,
        "ViewType"
        | "WorkspaceEvent"
        | "WsErrCode"
        | "WorkspaceObservable"
        | "EditorEvent"
        | "DocErrorCode"
        | "FFIStatusCode"
        | "UserStatus"
        | "UserEvent"
        | "UserErrCode"
        => TypeCategory::Enum,

        "Option" => TypeCategory::Opt,
        _ => TypeCategory::Primitive,
    }
}
