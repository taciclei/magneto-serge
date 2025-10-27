import {
  selectCassetteState,
  selectCassettes,
  selectSelectedCassette,
  selectTotalItems,
  selectPage,
  selectLimit,
  selectLoading,
  selectError,
  selectTotalPages,
  selectHasNextPage,
  selectHasPreviousPage,
  selectPaginationInfo,
  selectCassetteByName
} from './cassette.selectors';
import { CassetteState } from './cassette.reducer';
import { CassetteResource } from '../../../core/models/cassette.model';

describe('CassetteSelectors', () => {
  // Mock cassettes
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

  // Mock state
  const mockState: { cassettes: CassetteState } = {
    cassettes: {
      cassettes: mockCassettes,
      selectedCassette: mockCassettes[0],
      totalItems: 100,
      page: 2,
      limit: 20,
      loading: false,
      error: null
    }
  };

  describe('Feature Selector', () => {
    it('should select the cassette state', () => {
      const result = selectCassetteState(mockState);

      expect(result).toEqual(mockState.cassettes);
    });
  });

  describe('Basic Selectors', () => {
    it('should select cassettes', () => {
      const result = selectCassettes(mockState);

      expect(result).toEqual(mockCassettes);
    });

    it('should select selected cassette', () => {
      const result = selectSelectedCassette(mockState);

      expect(result).toEqual(mockCassettes[0]);
    });

    it('should select total items', () => {
      const result = selectTotalItems(mockState);

      expect(result).toBe(100);
    });

    it('should select page', () => {
      const result = selectPage(mockState);

      expect(result).toBe(2);
    });

    it('should select limit', () => {
      const result = selectLimit(mockState);

      expect(result).toBe(20);
    });

    it('should select loading', () => {
      const result = selectLoading(mockState);

      expect(result).toBe(false);
    });

    it('should select error', () => {
      const result = selectError(mockState);

      expect(result).toBeNull();
    });

    it('should select error when present', () => {
      const stateWithError = {
        cassettes: {
          ...mockState.cassettes,
          error: 'Test error'
        }
      };

      const result = selectError(stateWithError);

      expect(result).toBe('Test error');
    });
  });

  describe('Composed Selectors', () => {
    describe('selectTotalPages', () => {
      it('should calculate total pages correctly', () => {
        const result = selectTotalPages(mockState);

        expect(result).toBe(5); // 100 / 20 = 5
      });

      it('should round up for non-divisible totals', () => {
        const stateWithOddTotal = {
          cassettes: {
            ...mockState.cassettes,
            totalItems: 95
          }
        };

        const result = selectTotalPages(stateWithOddTotal);

        expect(result).toBe(5); // Math.ceil(95 / 20) = 5
      });

      it('should handle zero total items', () => {
        const stateWithZero = {
          cassettes: {
            ...mockState.cassettes,
            totalItems: 0
          }
        };

        const result = selectTotalPages(stateWithZero);

        expect(result).toBe(0);
      });

      it('should handle single item', () => {
        const stateWithOne = {
          cassettes: {
            ...mockState.cassettes,
            totalItems: 1,
            limit: 20
          }
        };

        const result = selectTotalPages(stateWithOne);

        expect(result).toBe(1);
      });
    });

    describe('selectHasNextPage', () => {
      it('should return true when there are more pages', () => {
        const result = selectHasNextPage(mockState);

        expect(result).toBe(true); // page 2 < totalPages 5
      });

      it('should return false when on last page', () => {
        const stateOnLastPage = {
          cassettes: {
            ...mockState.cassettes,
            page: 5
          }
        };

        const result = selectHasNextPage(stateOnLastPage);

        expect(result).toBe(false); // page 5 = totalPages 5
      });

      it('should return false when past last page', () => {
        const statePastLastPage = {
          cassettes: {
            ...mockState.cassettes,
            page: 10
          }
        };

        const result = selectHasNextPage(statePastLastPage);

        expect(result).toBe(false); // page 10 > totalPages 5
      });
    });

    describe('selectHasPreviousPage', () => {
      it('should return true when not on first page', () => {
        const result = selectHasPreviousPage(mockState);

        expect(result).toBe(true); // page 2 > 1
      });

      it('should return false when on first page', () => {
        const stateOnFirstPage = {
          cassettes: {
            ...mockState.cassettes,
            page: 1
          }
        };

        const result = selectHasPreviousPage(stateOnFirstPage);

        expect(result).toBe(false); // page 1
      });
    });

    describe('selectPaginationInfo', () => {
      it('should combine all pagination info', () => {
        const result = selectPaginationInfo(mockState);

        expect(result).toEqual({
          page: 2,
          limit: 20,
          totalItems: 100,
          totalPages: 5,
          hasNext: true,
          hasPrevious: true,
          startIndex: 21, // (2-1) * 20 + 1
          endIndex: 40   // 2 * 20
        });
      });

      it('should calculate startIndex and endIndex correctly for first page', () => {
        const stateFirstPage = {
          cassettes: {
            ...mockState.cassettes,
            page: 1
          }
        };

        const result = selectPaginationInfo(stateFirstPage);

        expect(result.startIndex).toBe(1); // (1-1) * 20 + 1
        expect(result.endIndex).toBe(20); // 1 * 20
      });

      it('should calculate endIndex correctly for last page with fewer items', () => {
        const stateLastPage = {
          cassettes: {
            ...mockState.cassettes,
            page: 5,
            totalItems: 95 // Last page has 15 items
          }
        };

        const result = selectPaginationInfo(stateLastPage);

        expect(result.startIndex).toBe(81); // (5-1) * 20 + 1
        expect(result.endIndex).toBe(95);  // Math.min(5 * 20, 95) = 95
      });

      it('should handle empty results', () => {
        const stateEmpty = {
          cassettes: {
            ...mockState.cassettes,
            totalItems: 0,
            page: 1
          }
        };

        const result = selectPaginationInfo(stateEmpty);

        expect(result.startIndex).toBe(1);
        expect(result.endIndex).toBe(0); // Math.min(20, 0) = 0
      });
    });
  });

  describe('selectCassetteByName', () => {
    it('should find cassette by name', () => {
      const selector = selectCassetteByName('cassette-1');
      const result = selector(mockState);

      expect(result).toEqual(mockCassettes[0]);
    });

    it('should find second cassette by name', () => {
      const selector = selectCassetteByName('cassette-2');
      const result = selector(mockState);

      expect(result).toEqual(mockCassettes[1]);
    });

    it('should return undefined when cassette not found', () => {
      const selector = selectCassetteByName('nonexistent');
      const result = selector(mockState);

      expect(result).toBeUndefined();
    });

    it('should return undefined when cassettes array is empty', () => {
      const stateEmpty = {
        cassettes: {
          ...mockState.cassettes,
          cassettes: []
        }
      };

      const selector = selectCassetteByName('cassette-1');
      const result = selector(stateEmpty);

      expect(result).toBeUndefined();
    });
  });

  describe('Selector Memoization', () => {
    it('should return same result for same state (memoization)', () => {
      const result1 = selectCassettes(mockState);
      const result2 = selectCassettes(mockState);

      expect(result1).toBe(result2); // Should be same reference
    });

    it('should return different result for different state', () => {
      const result1 = selectCassettes(mockState);

      const modifiedState = {
        cassettes: {
          ...mockState.cassettes,
          cassettes: [...mockCassettes, {
            '@id': '/api/cassettes/cassette-3',
            '@type': 'magneto:Cassette',
            name: 'cassette-3',
            version: '1.0',
            recordedAt: '2025-10-27T12:00:00Z',
            interactionCount: 3,
            sizeBytes: 512,
            hasTemplates: false
          }]
        }
      };

      const result2 = selectCassettes(modifiedState);

      expect(result1).not.toBe(result2); // Should be different reference
      expect(result2.length).toBe(3);
    });
  });

  describe('Edge Cases', () => {
    it('should handle state with null selectedCassette', () => {
      const stateNoSelection = {
        cassettes: {
          ...mockState.cassettes,
          selectedCassette: null
        }
      };

      const result = selectSelectedCassette(stateNoSelection);

      expect(result).toBeNull();
    });

    it('should handle very large pagination numbers', () => {
      const stateLargeNumbers = {
        cassettes: {
          ...mockState.cassettes,
          totalItems: 10000,
          page: 250,
          limit: 50
        }
      };

      const result = selectPaginationInfo(stateLargeNumbers);

      expect(result.totalPages).toBe(200); // 10000 / 50
      expect(result.startIndex).toBe(12451); // (250-1) * 50 + 1
      expect(result.endIndex).toBe(10000); // Math.min(250 * 50, 10000)
      expect(result.hasNext).toBe(false); // 250 > 200
      expect(result.hasPrevious).toBe(true);
    });

    it('should handle limit of 1', () => {
      const stateLimit1 = {
        cassettes: {
          ...mockState.cassettes,
          totalItems: 100,
          limit: 1,
          page: 50
        }
      };

      const result = selectPaginationInfo(stateLimit1);

      expect(result.totalPages).toBe(100);
      expect(result.startIndex).toBe(50);
      expect(result.endIndex).toBe(50);
    });

    it('should handle loading state', () => {
      const stateLoading = {
        cassettes: {
          ...mockState.cassettes,
          loading: true
        }
      };

      const result = selectLoading(stateLoading);

      expect(result).toBe(true);
    });
  });
});
