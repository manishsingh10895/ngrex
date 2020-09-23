use colored::*;
use exitfailure::ExitFailure;
use inflector::Inflector;
use std::fs::{self, File, OpenOptions};
use std::io::Write;

const STORE_PATH: &str = "./src/app/store";

pub fn get_capitalized_feature(feature: &str) -> String {
    feature.to_title_case()
}

pub fn action_template() -> String {
    return String::from("import { createAction } from '@ngrx/store';");
}

pub fn reducer_template(feature: &str) -> String {
    let cap = get_capitalized_feature(feature);

    return format!(
        r#"
import * as  {cap}Actions from '../actions/{feature}.actions';
import {{ createReducer, on, State, Action }} from '@ngrx/store';

export interface {cap}State {{

}}

export const Initial%sState: %sState = {{

}}	

const reducer = createReducer(
	Initial{cap}State,

	)

export function {feature}Reducer(state: %sState | undefined, action: Action) {{
	return reducer(state, action)
}}
	"#,
        cap = cap,
        feature = feature
    );
}

pub fn selector_template(feature: &str) -> String {
    let cap = get_capitalized_feature(feature);

    return format!(
        r#"
import {{ createSelector, createFeatureSelector }} from "@ngrx/store";
import {{ {cap}State }} from '../reducers/{feature}.reducer';
import {{ RootState }} from '../store';

export const {cap}Selector = createFeatureSelector<RootState, {cap}State>('%s')
	"#,
        cap = cap,
        feature = feature
    );
}

pub fn effect_template(feature: &str) -> String {
    let cap = crate::helper::get_capitalized_feature(feature);

    return format!(
        r#"
import {{ Injectable }} from '@angular/core';
import {{ Actions, createEffect, ofType }} from '@ngrx/effects';
import {{ map, mergeMap, catchError }} from 'rxjs/operators';

@Injectable()
export class {}Effects {{
	newAction$ = createEffect(() => this.actions$.pipe(

	)
	);

	constructor(
		private actions$: Actions,
	) {{}}
}}"#,
        cap
    );
}

fn get_component_template(feature: &str, component: &str) -> String {
    match component {
        "action" => action_template(),
        "effect" => effect_template(feature),
        "selector" => selector_template(feature),
        "reducer" => reducer_template(feature),
        _ => panic!("Invalid component type {}", component),
    }
}

pub fn create_store_component(feature: &str, component: &str) -> Result<(), ExitFailure> {
    let mut file: File;

    let filename = get_component_file_path(feature, component);

    let template = get_component_template(feature, component);

    println!("{}", filename);

    if std::fs::metadata(&filename).is_err() {
        let p = std::path::Path::new(&filename);

        let dir = p.parent().unwrap();

        if fs::metadata(dir).is_err() {
            fs::create_dir_all(dir)?;
        }
    }

    file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&filename)?;

    file.write_all(template.as_bytes())?;

    let msg = format!("Created {} at {}", component, &filename);

    println!("{}", msg.green());

    return Ok(());
}

pub fn get_component_file_path(feature: &str, component: &str) -> String {
    match component {
        "action" => format!(
            "{root}/actions/{feature}.actions.ts",
            root = STORE_PATH,
            feature = feature
        ),
        "effect" => format!("{}/effects/{}.effect.ts", STORE_PATH, feature),
        "selector" => format!("{}/selectors/{}.selectors.ts", STORE_PATH, feature),
        "reducer" => format!("{}/reducers/{}.reducer.ts", STORE_PATH, feature),
        _ => panic!("Invalid component type"),
    }
}
