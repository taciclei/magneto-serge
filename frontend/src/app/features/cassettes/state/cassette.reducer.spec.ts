import { cassettesReducer, initialState, CassetteState } from './cassette.reducer';
import { CassetteActions } from './cassette.actions';
import { CassetteResource } from '../../../core/models/cassette.model';

describe('CassetteReducer', () => {
  // Mock cassettes for testing
  const mockCassettes: CassetteResource[] = [
    {
      '@id': '/api/cassettes/cassette-1',
      '@type': 'magneto:Cassette',
      name: 'cassette-1',
      version: '1.0',
      recordedAt: '2025-10-27T10:00:00Z',
      interactionCount: 10,
      sizeBytes: 2048,
      hasTemplates: false
    },
    {
      '@id': '/api/cassettes/cassette-2',
      '@type': 'magneto:Cassette',
      name: 'cassette-2',
      version: '1.0',
      recordedAt: '2025-10-27T11:00:00Z',
      interactionCount: 5,
      sizeBytes: 1024,
      hasTemplates: true
    }
  ];

  describe('Initial State', () => {
    it('should have the correct initial state', () => {
      expect(initialState).toEqual({
        cassettes: [],
        selectedCassette: null,
        totalItems: 0,
        page: 1,
        limit: 20,
        loading: false,
        error: null
      });
    });

    it('should return initial state when called with undefined state', () => {
      const action = { type: 'NOOP' };
      const state = cassettesReducer(undefined, action as any);

      expect(state).toEqual(initialState);
    });
  });

  describe('Load Cassettes Actions', () => {
    it('should set loading=true and clear error on loadCassettes', () => {
      const previousState: CassetteState = {
        ...initialState,
        error: 'Previous error'
      };

      const action = CassetteActions.loadCassettes({ params: { page: 2, limit: 10 } });
      const state = cassettesReducer(previousState, action);

      expect(state.loading).toBe(true);
      expect(state.error).toBeNull();
      expect(state.page).toBe(2);
      expect(state.limit).toBe(10);
    });

    it('should use default page and limit if params not provided', () => {
      const previousState: CassetteState = {
        ...initialState,
        page: 3,
        limit: 50
      };

      const action = CassetteActions.loadCassettes({});
      const state = cassettesReducer(previousState, action);

      expect(state.page).toBe(3); // Keep previous page
      expect(state.limit).toBe(50); // Keep previous limit
    });

    it('should load cassettes successfully', () => {
      const action = CassetteActions.loadCassettesSuccess({
        cassettes: mockCassettes,
        totalItems: 25,
        page: 2,
        limit: 10
      });

      const state = cassettesReducer(initialState, action);

      expect(state.cassettes).toEqual(mockCassettes);
      expect(state.totalItems).toBe(25);
      expect(state.page).toBe(2);
      expect(state.limit).toBe(10);
      expect(state.loading).toBe(false);
      expect(state.error).toBeNull();
    });

    it('should handle load cassettes failure', () => {
      const errorMessage = 'Failed to load cassettes';
      const previousState: CassetteState = {
        ...initialState,
        loading: true
      };

      const action = CassetteActions.loadCassettesFailure({ error: errorMessage });
      const state = cassettesReducer(previousState, action);

      expect(state.loading).toBe(false);
      expect(state.error).toBe(errorMessage);
      expect(state.cassettes).toEqual([]); // Cassettes should remain unchanged
    });
  });

  describe('Navigation Actions', () => {
    it('should navigate to specific page', () => {
      const action = CassetteActions.navigateToPage({ page: 5 });
      const state = cassettesReducer(initialState, action);

      expect(state.page).toBe(5);
      expect(state.loading).toBe(true);
    });

    it('should navigate to next page', () => {
      const previousState: CassetteState = {
        ...initialState,
        page: 2
      };

      const action = CassetteActions.navigateToNextPage();
      const state = cassettesReducer(previousState, action);

      expect(state.page).toBe(3);
      expect(state.loading).toBe(true);
    });

    it('should navigate to previous page', () => {
      const previousState: CassetteState = {
        ...initialState,
        page: 3
      };

      const action = CassetteActions.navigateToPreviousPage();
      const state = cassettesReducer(previousState, action);

      expect(state.page).toBe(2);
      expect(state.loading).toBe(true);
    });

    it('should not go below page 1 when navigating to previous page', () => {
      const previousState: CassetteState = {
        ...initialState,
        page: 1
      };

      const action = CassetteActions.navigateToPreviousPage();
      const state = cassettesReducer(previousState, action);

      expect(state.page).toBe(1); // Should stay at 1
    });

    it('should navigate to first page', () => {
      const previousState: CassetteState = {
        ...initialState,
        page: 10
      };

      const action = CassetteActions.navigateToFirstPage();
      const state = cassettesReducer(previousState, action);

      expect(state.page).toBe(1);
      expect(state.loading).toBe(true);
    });

    it('should navigate to last page', () => {
      const previousState: CassetteState = {
        ...initialState,
        totalItems: 100,
        limit: 20,
        page: 1
      };

      const action = CassetteActions.navigateToLastPage();
      const state = cassettesReducer(previousState, action);

      expect(state.page).toBe(5); // 100 / 20 = 5
      expect(state.loading).toBe(true);
    });

    it('should handle last page calculation with non-divisible totalItems', () => {
      const previousState: CassetteState = {
        ...initialState,
        totalItems: 95,
        limit: 20
      };

      const action = CassetteActions.navigateToLastPage();
      const state = cassettesReducer(previousState, action);

      expect(state.page).toBe(5); // Math.ceil(95 / 20) = 5
    });
  });

  describe('Load Cassette Detail Actions', () => {
    it('should set loading=true and clear error on loadCassetteDetail', () => {
      const previousState: CassetteState = {
        ...initialState,
        error: 'Previous error'
      };

      const action = CassetteActions.loadCassetteDetail({ name: 'test-cassette' });
      const state = cassettesReducer(previousState, action);

      expect(state.loading).toBe(true);
      expect(state.error).toBeNull();
    });

    it('should load cassette detail successfully', () => {
      const cassette = mockCassettes[0];
      const action = CassetteActions.loadCassetteDetailSuccess({ cassette });
      const state = cassettesReducer(initialState, action);

      expect(state.selectedCassette).toEqual(cassette);
      expect(state.loading).toBe(false);
      expect(state.error).toBeNull();
    });

    it('should handle load cassette detail failure', () => {
      const errorMessage = 'Cassette not found';
      const previousState: CassetteState = {
        ...initialState,
        loading: true
      };

      const action = CassetteActions.loadCassetteDetailFailure({ error: errorMessage });
      const state = cassettesReducer(previousState, action);

      expect(state.loading).toBe(false);
      expect(state.error).toBe(errorMessage);
      expect(state.selectedCassette).toBeNull();
    });
  });

  describe('Selection Actions', () => {
    it('should select cassette by name from cassettes list', () => {
      const previousState: CassetteState = {
        ...initialState,
        cassettes: mockCassettes
      };

      const action = CassetteActions.selectCassette({ name: 'cassette-1' });
      const state = cassettesReducer(previousState, action);

      expect(state.selectedCassette).toEqual(mockCassettes[0]);
    });

    it('should set selectedCassette to null if name not found', () => {
      const previousState: CassetteState = {
        ...initialState,
        cassettes: mockCassettes
      };

      const action = CassetteActions.selectCassette({ name: 'nonexistent' });
      const state = cassettesReducer(previousState, action);

      expect(state.selectedCassette).toBeNull();
    });

    it('should clear selection', () => {
      const previousState: CassetteState = {
        ...initialState,
        selectedCassette: mockCassettes[0]
      };

      const action = CassetteActions.clearSelection();
      const state = cassettesReducer(previousState, action);

      expect(state.selectedCassette).toBeNull();
    });
  });

  describe('Complex State Transitions', () => {
    it('should handle sequence of actions correctly', () => {
      // Start with initial state
      let state = initialState;

      // Load cassettes
      state = cassettesReducer(state, CassetteActions.loadCassettes({ params: { page: 1, limit: 20 } }));
      expect(state.loading).toBe(true);

      // Success
      state = cassettesReducer(state, CassetteActions.loadCassettesSuccess({
        cassettes: mockCassettes,
        totalItems: 50,
        page: 1,
        limit: 20
      }));
      expect(state.loading).toBe(false);
      expect(state.cassettes.length).toBe(2);

      // Select cassette
      state = cassettesReducer(state, CassetteActions.selectCassette({ name: 'cassette-1' }));
      expect(state.selectedCassette).toEqual(mockCassettes[0]);

      // Navigate to next page
      state = cassettesReducer(state, CassetteActions.navigateToNextPage());
      expect(state.page).toBe(2);
      expect(state.loading).toBe(true);

      // Clear selection
      state = cassettesReducer(state, CassetteActions.clearSelection());
      expect(state.selectedCassette).toBeNull();
    });

    it('should preserve cassettes when navigation fails', () => {
      const previousState: CassetteState = {
        ...initialState,
        cassettes: mockCassettes,
        totalItems: 50
      };

      const action = CassetteActions.navigateToNextPage();
      const state = cassettesReducer(previousState, action);

      expect(state.cassettes).toEqual(mockCassettes); // Should preserve cassettes
    });
  });

  describe('Edge Cases', () => {
    it('should handle empty cassettes array', () => {
      const action = CassetteActions.loadCassettesSuccess({
        cassettes: [],
        totalItems: 0,
        page: 1,
        limit: 20
      });

      const state = cassettesReducer(initialState, action);

      expect(state.cassettes).toEqual([]);
      expect(state.totalItems).toBe(0);
    });

    it('should handle large page numbers', () => {
      const action = CassetteActions.navigateToPage({ page: 9999 });
      const state = cassettesReducer(initialState, action);

      expect(state.page).toBe(9999);
    });

    it('should handle zero totalItems when navigating to last page', () => {
      const previousState: CassetteState = {
        ...initialState,
        totalItems: 0,
        limit: 20
      };

      const action = CassetteActions.navigateToLastPage();
      const state = cassettesReducer(previousState, action);

      // Math.ceil(0 / 20) = 0, but page should be at least 0
      expect(state.page).toBe(0);
    });

    it('should handle multiple consecutive failures', () => {
      let state = initialState;

      state = cassettesReducer(state, CassetteActions.loadCassettesFailure({ error: 'Error 1' }));
      expect(state.error).toBe('Error 1');

      state = cassettesReducer(state, CassetteActions.loadCassettesFailure({ error: 'Error 2' }));
      expect(state.error).toBe('Error 2');

      state = cassettesReducer(state, CassetteActions.loadCassettesFailure({ error: 'Error 3' }));
      expect(state.error).toBe('Error 3');
    });
  });
});
