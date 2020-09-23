
import { createSelector, createFeatureSelector } from "@ngrx/store";
import { ChartState } from '../reducers/chart.reducer';
import { RootState } from '../store';

export const ChartSelector = createFeatureSelector<RootState, ChartState>('%s')
	