use tauri::{command, State};

use crate::{
    application::{
        CreateCylinderMaterialInput, CreateCylinderMaterialUseCase, ListCylinderMaterialsUseCase,
        DeleteCylinderMaterialInput, DeleteCylinderMaterialUseCase, SolveCylinderWeightInput,
        ExportCylinderMaterialsUseCase, ImportCylinderMaterialsInput, ImportCylinderMaterialsUseCase,
        SolveCylinderWeightUseCase, UpdateCylinderMaterialInput, UpdateCylinderMaterialUseCase,
    },
    interface::tauri::error::{map_application_error, TauriError, TauriFieldError},
    AppState,
};

use super::{
    CreateCylinderMaterialRequest, CylinderMaterialResponse, DeleteCylinderMaterialRequest,
    ExportCylinderMaterialsResponse, ImportCylinderMaterialsRequest, ImportCylinderMaterialsResponse,
    SolveCylinderWeightRequest,
    SolveCylinderWeightResponse,
    UpdateCylinderMaterialRequest,
};

#[command]
pub fn list_cylinder_materials(
    state: State<AppState>,
) -> Result<Vec<CylinderMaterialResponse>, TauriError> {
    let repo = state.cylinder_material_repository.lock().unwrap();
    let rows = ListCylinderMaterialsUseCase::execute(&*repo);
    Ok(rows.into_iter().map(Into::into).collect())
}

#[command]
pub fn create_cylinder_material(
    state: State<AppState>,
    request: CreateCylinderMaterialRequest,
) -> Result<CylinderMaterialResponse, TauriError> {
    let input: CreateCylinderMaterialInput = request.into();
    let mut repo = state.cylinder_material_repository.lock().unwrap();

    match CreateCylinderMaterialUseCase::execute(&mut *repo, input) {
        Ok(out) => Ok(out.into()),
        Err(crate::application::ApplicationError::Infrastructure(msg)) if msg == "duplicate_material" => {
            Err(TauriError {
                message: "Material already exists".to_string(),
                field_errors: Some(vec![TauriFieldError {
                    field: "name".to_string(),
                    code: "duplicate_material".to_string(),
                    message: "material name already exists".to_string(),
                }]),
            })
        }
        Err(err) => Err(map_application_error(err)),
    }
}

#[command]
pub fn solve_cylinder_weight(
    state: State<AppState>,
    request: SolveCylinderWeightRequest,
) -> Result<SolveCylinderWeightResponse, TauriError> {
    let input: SolveCylinderWeightInput = request.into();
    let repo = state.cylinder_material_repository.lock().unwrap();

    let out = SolveCylinderWeightUseCase::execute(&*repo, input).map_err(map_application_error)?;
    Ok(out.into())
}

#[command]
pub fn update_cylinder_material(
    state: State<AppState>,
    request: UpdateCylinderMaterialRequest,
) -> Result<CylinderMaterialResponse, TauriError> {
    let input: UpdateCylinderMaterialInput = request.into();
    let mut repo = state.cylinder_material_repository.lock().unwrap();

    match UpdateCylinderMaterialUseCase::execute(&mut *repo, input) {
        Ok(out) => Ok(out.into()),
        Err(crate::application::ApplicationError::Infrastructure(msg)) if msg == "duplicate_material" => {
            Err(TauriError {
                message: "Material already exists".to_string(),
                field_errors: Some(vec![TauriFieldError {
                    field: "name".to_string(),
                    code: "duplicate_material".to_string(),
                    message: "material name already exists".to_string(),
                }]),
            })
        }
        Err(crate::application::ApplicationError::Infrastructure(msg)) if msg == "material_not_found" => {
            Err(TauriError {
                message: "Material not found".to_string(),
                field_errors: Some(vec![TauriFieldError {
                    field: "id".to_string(),
                    code: "material_not_found".to_string(),
                    message: "material was not found".to_string(),
                }]),
            })
        }
        Err(err) => Err(map_application_error(err)),
    }
}

#[command]
pub fn delete_cylinder_material(
    state: State<AppState>,
    request: DeleteCylinderMaterialRequest,
) -> Result<(), TauriError> {
    let input: DeleteCylinderMaterialInput = request.into();
    let mut repo = state.cylinder_material_repository.lock().unwrap();

    match DeleteCylinderMaterialUseCase::execute(&mut *repo, input) {
        Ok(()) => Ok(()),
        Err(crate::application::ApplicationError::Infrastructure(msg)) if msg == "material_not_found" => {
            Err(TauriError {
                message: "Material not found".to_string(),
                field_errors: Some(vec![TauriFieldError {
                    field: "id".to_string(),
                    code: "material_not_found".to_string(),
                    message: "material was not found".to_string(),
                }]),
            })
        }
        Err(err) => Err(map_application_error(err)),
    }
}

#[command]
pub fn import_cylinder_materials(
    state: State<AppState>,
    request: ImportCylinderMaterialsRequest,
) -> Result<ImportCylinderMaterialsResponse, TauriError> {
    let input: ImportCylinderMaterialsInput = request.into();
    let mut repo = state.cylinder_material_repository.lock().unwrap();

    let out = ImportCylinderMaterialsUseCase::execute(&mut *repo, input)
        .map_err(map_application_error)?;
    Ok(out.into())
}

#[command]
pub fn export_cylinder_materials(
    state: State<AppState>,
) -> Result<ExportCylinderMaterialsResponse, TauriError> {
    let repo = state.cylinder_material_repository.lock().unwrap();
    let out = ExportCylinderMaterialsUseCase::execute(&*repo);
    Ok(out.into())
}
