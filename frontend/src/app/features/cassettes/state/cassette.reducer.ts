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
  search: string | null;
  filterType: string | null;
  sortBy: string | null;
  sortOrder: string;
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
  search: null,
  filterType: null,
  sortBy: null,
  sortOrder: 'asc',
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
    limit: params?.limit ?? state.limit,
    search: params?.search ?? state.search,
    filterType: params?.filter_type ?? state.filterType,
    sortBy: params?.sort_by ?? state.sortBy,
    sortOrder: params?.sort_order ?? state.sortOrder
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
  })),

  // Recherche et tri
  on(CassetteActions.updateSearch, (state, { search }) => ({
    ...state,
    search,
    page: 1, // Reset to page 1 when search changes
    loading: true
  })),

  on(CassetteActions.updateFilterType, (state, { filterType }) => ({
    ...state,
    filterType,
    page: 1, // Reset to page 1 when filter changes
    loading: true
  })),

  on(CassetteActions.updateSort, (state, { sortBy, sortOrder }) => ({
    ...state,
    sortBy,
    sortOrder: sortOrder ?? state.sortOrder,
    loading: true
  })),

  on(CassetteActions.clearFilters, (state) => ({
    ...state,
    search: null,
    filterType: null,
    sortBy: null,
    sortOrder: 'asc',
    page: 1,
    loading: true
  }))
);
