use tauri::{command, State};

use crate::{
    application::{
        CreateCylinderMaterialInput, CreateCylinderMaterialUseCase, ListCylinderMaterialsUseCase,
        SolveCylinderWeightInput, SolveCylinderWeightUseCase,
    },
    interface::tauri::error::{map_application_error, TauriError, TauriFieldError},
    AppState,
};

use super::{
    CreateCylinderMaterialRequest, CylinderMaterialResponse, SolveCylinderWeightRequest,
    SolveCylinderWeightResponse,
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
