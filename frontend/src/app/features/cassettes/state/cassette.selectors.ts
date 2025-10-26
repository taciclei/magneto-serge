import { createFeatureSelector, createSelector } from '@ngrx/store';
import { CassetteState } from './cassette.reducer';

/**
 * Selectors NgRx pour les cassettes
 *
 * Permettent de sélectionner des portions spécifiques du state
 */

// Sélecteur de feature
export const selectCassetteState = createFeatureSelector<CassetteState>('cassettes');

// Selectors de base
export const selectCassettes = createSelector(
  selectCassetteState,
  (state) => state.cassettes
);

export const selectSelectedCassette = createSelector(
  selectCassetteState,
  (state) => state.selectedCassette
);

export const selectTotalItems = createSelector(
  selectCassetteState,
  (state) => state.totalItems
);

export const selectPage = createSelector(
  selectCassetteState,
  (state) => state.page
);

export const selectLimit = createSelector(
  selectCassetteState,
  (state) => state.limit
);

export const selectLoading = createSelector(
  selectCassetteState,
  (state) => state.loading
);

export const selectError = createSelector(
  selectCassetteState,
  (state) => state.error
);

export const selectSearch = createSelector(
  selectCassetteState,
  (state) => state.search
);

export const selectFilterType = createSelector(
  selectCassetteState,
  (state) => state.filterType
);

export const selectSortBy = createSelector(
  selectCassetteState,
  (state) => state.sortBy
);

export const selectSortOrder = createSelector(
  selectCassetteState,
  (state) => state.sortOrder
);

// Selectors composés
export const selectTotalPages = createSelector(
  selectTotalItems,
  selectLimit,
  (totalItems, limit) => Math.ceil(totalItems / limit)
);

export const selectHasNextPage = createSelector(
  selectPage,
  selectTotalPages,
  (page, totalPages) => page < totalPages
);

export const selectHasPreviousPage = createSelector(
  selectPage,
  (page) => page > 1
);

export const selectPaginationInfo = createSelector(
  selectPage,
  selectLimit,
  selectTotalItems,
  selectTotalPages,
  selectHasNextPage,
  selectHasPreviousPage,
  (page, limit, totalItems, totalPages, hasNext, hasPrevious) => ({
    page,
    limit,
    totalItems,
    totalPages,
    hasNext,
    hasPrevious,
    startIndex: (page - 1) * limit + 1,
    endIndex: Math.min(page * limit, totalItems)
  })
);

// Selector pour une cassette spécifique par nom
export const selectCassetteByName = (name: string) =>
  createSelector(
    selectCassettes,
    (cassettes) => cassettes.find(c => c.name === name)
  );
