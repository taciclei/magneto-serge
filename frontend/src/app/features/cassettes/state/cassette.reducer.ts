import { createReducer, on } from '@ngrx/store';
import { CassetteResource } from '../../../core/models/cassette.model';
import { CassetteActions } from './cassette.actions';

/**
 * État NgRx pour les cassettes
 */
export interface CassetteState {
  cassettes: CassetteResource[];
  selectedCassette: CassetteResource | null;
  totalItems: number;
  page: number;
  limit: number;
  loading: boolean;
  error: string | null;
}

/**
 * État initial
 */
export const initialState: CassetteState = {
  cassettes: [],
  selectedCassette: null,
  totalItems: 0,
  page: 1,
  limit: 20,
  loading: false,
  error: null
};

/**
 * Reducer NgRx pour les cassettes
 */
export const cassettesReducer = createReducer(
  initialState,

  // Load Cassettes
  on(CassetteActions.loadCassettes, (state, { params }) => ({
    ...state,
    loading: true,
    error: null,
    page: params?.page ?? state.page,
    limit: params?.limit ?? state.limit
  })),

  on(CassetteActions.loadCassettesSuccess, (state, { cassettes, totalItems, page, limit }) => ({
    ...state,
    cassettes,
    totalItems,
    page,
    limit,
    loading: false,
    error: null
  })),

  on(CassetteActions.loadCassettesFailure, (state, { error }) => ({
    ...state,
    loading: false,
    error
  })),

  // Navigate to Page
  on(CassetteActions.navigateToPage, (state, { page }) => ({
    ...state,
    page,
    loading: true
  })),

  on(CassetteActions.navigateToNextPage, (state) => ({
    ...state,
    page: state.page + 1,
    loading: true
  })),

  on(CassetteActions.navigateToPreviousPage, (state) => ({
    ...state,
    page: Math.max(1, state.page - 1),
    loading: true
  })),

  on(CassetteActions.navigateToFirstPage, (state) => ({
    ...state,
    page: 1,
    loading: true
  })),

  on(CassetteActions.navigateToLastPage, (state) => {
    const lastPage = Math.ceil(state.totalItems / state.limit);
    return {
      ...state,
      page: lastPage,
      loading: true
    };
  }),

  // Load Cassette Detail
  on(CassetteActions.loadCassetteDetail, (state) => ({
    ...state,
    loading: true,
    error: null
  })),

  on(CassetteActions.loadCassetteDetailSuccess, (state, { cassette }) => ({
    ...state,
    selectedCassette: cassette,
    loading: false,
    error: null
  })),

  on(CassetteActions.loadCassetteDetailFailure, (state, { error }) => ({
    ...state,
    loading: false,
    error
  })),

  // Selection
  on(CassetteActions.selectCassette, (state, { name }) => {
    const cassette = state.cassettes.find(c => c.name === name) || null;
    return {
      ...state,
      selectedCassette: cassette
    };
  }),

  on(CassetteActions.clearSelection, (state) => ({
    ...state,
    selectedCassette: null
  }))
);
