
import * as  ChartActions from '../actions/chart.actions';
import { createReducer, on, State, Action } from '@ngrx/store';

export interface ChartState {

}

export const Initial%sState: %sState = {

}	

const reducer = createReducer(
	InitialChartState,

	)

export function chartReducer(state: %sState | undefined, action: Action) {
	return reducer(state, action)
}
	