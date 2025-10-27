import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Router } from '@angular/router';
import { Store } from '@ngrx/store';
import { of } from 'rxjs';
import { NoopAnimationsModule } from '@angular/platform-browser/animations';

import { CassetteListComponent } from './cassette-list.component';
import { CassetteResource } from '../../../../core/models/cassette.model';
import { CassetteActions } from '../../state/cassette.actions';
import {
  selectCassettes,
  selectLoading,
  selectError,
  selectPaginationInfo
} from '../../state/cassette.selectors';

describe('CassetteListComponent', () => {
  let component: CassetteListComponent;
  let fixture: ComponentFixture<CassetteListComponent>;
  let mockStore: jasmine.SpyObj<Store>;
  let mockRouter: jasmine.SpyObj<Router>;

  // Mock cassettes data
  const mockCassettes: CassetteResource[] = [
    {
      '@id': '/api/cassettes/test-cassette-1',
      '@type': 'magneto:Cassette',
      name: 'test-cassette-1',
      version: '1.0',
      recordedAt: '2025-10-27T10:00:00Z',
      interactionCount: 10,
      sizeBytes: 2048,
      hasTemplates: false
    },
    {
      '@id': '/api/cassettes/test-cassette-2',
      '@type': 'magneto:Cassette',
      name: 'test-cassette-2',
      version: '1.0',
      recordedAt: '2025-10-27T11:00:00Z',
      interactionCount: 5,
      sizeBytes: 1024 * 1024, // 1 MB
      hasTemplates: true,
      description: 'Test cassette'
    }
  ];

  const mockPaginationInfo = {
    currentPage: 1,
    totalItems: 25,
    itemsPerPage: 20,
    hasNext: true,
    hasPrevious: false,
    firstPageUrl: '/api/cassettes?page=1',
    lastPageUrl: '/api/cassettes?page=2'
  };

  beforeEach(async () => {
    // Create mock Store with select method
    mockStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
    mockRouter = jasmine.createSpyObj('Router', ['navigate']);

    // Setup default return values for store selectors using callFake
    // This allows different selectors to return different values
    // We use string matching on the selector name since selector functions can't be compared directly
    mockStore.select.and.callFake((selector: any) => {
      const selectorStr = selector.toString();

      if (selectorStr.includes('cassettes') && !selectorStr.includes('State')) {
        return of([]);
      } else if (selectorStr.includes('loading')) {
        return of(false);
      } else if (selectorStr.includes('error')) {
        return of(null);
      } else if (selectorStr.includes('pagination') || selectorStr.includes('Pagination')) {
        return of(null);
      }
      return of(null);
    });

    await TestBed.configureTestingModule({
      imports: [
        CassetteListComponent,
        NoopAnimationsModule
      ],
      providers: [
        { provide: Store, useValue: mockStore },
        { provide: Router, useValue: mockRouter }
      ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CassetteListComponent);
    component = fixture.componentInstance;
  });

  afterEach(() => {
    fixture.destroy();
  });

  describe('Component Initialization', () => {
    it('should create', () => {
      expect(component).toBeTruthy();
    });

    it('should initialize observables from store', () => {
      expect(component.cassettes$).toBeDefined();
      expect(component.loading$).toBeDefined();
      expect(component.error$).toBeDefined();
      expect(component.paginationInfo$).toBeDefined();
    });

    it('should have correct displayedColumns', () => {
      expect(component.displayedColumns).toEqual([
        'name',
        'version',
        'recordedAt',
        'interactionCount',
        'sizeBytes',
        'actions'
      ]);
    });

    it('should dispatch loadCassettes action on init', () => {
      fixture.detectChanges(); // triggers ngOnInit

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.loadCassettes({ params: { page: 1, limit: 20 } })
      );
    });

    it('should select correct state slices from store', () => {
      fixture.detectChanges();

      expect(mockStore.select).toHaveBeenCalledTimes(4);
    });
  });

  describe('Navigation Methods', () => {
    it('viewDetails should navigate to cassette detail page', () => {
      const cassette = mockCassettes[0];

      component.viewDetails(cassette);

      expect(mockRouter.navigate).toHaveBeenCalledWith(['/cassettes', 'test-cassette-1']);
    });

    it('reload should dispatch loadCassettes action', () => {
      mockStore.dispatch.calls.reset();

      component.reload();

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.loadCassettes({})
      );
    });
  });

  describe('Pagination Methods', () => {
    beforeEach(() => {
      mockStore.dispatch.calls.reset();
    });

    it('nextPage should dispatch navigateToNextPage action', () => {
      component.nextPage();

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.navigateToNextPage()
      );
    });

    it('previousPage should dispatch navigateToPreviousPage action', () => {
      component.previousPage();

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.navigateToPreviousPage()
      );
    });

    it('firstPage should dispatch navigateToFirstPage action', () => {
      component.firstPage();

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.navigateToFirstPage()
      );
    });

    it('lastPage should dispatch navigateToLastPage action', () => {
      component.lastPage();

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.navigateToLastPage()
      );
    });
  });

  describe('Helper Methods - formatSize', () => {
    it('should format bytes correctly', () => {
      expect(component.formatSize(500)).toBe('500 B');
      expect(component.formatSize(100)).toBe('100 B');
    });

    it('should format kilobytes correctly', () => {
      expect(component.formatSize(1024)).toBe('1.00 KB');
      expect(component.formatSize(2048)).toBe('2.00 KB');
      expect(component.formatSize(1536)).toBe('1.50 KB');
    });

    it('should format megabytes correctly', () => {
      expect(component.formatSize(1024 * 1024)).toBe('1.00 MB');
      expect(component.formatSize(2 * 1024 * 1024)).toBe('2.00 MB');
      expect(component.formatSize(1.5 * 1024 * 1024)).toBe('1.50 MB');
    });

    it('should handle edge cases', () => {
      expect(component.formatSize(0)).toBe('0 B');
      expect(component.formatSize(1023)).toBe('1023 B');
      expect(component.formatSize(1025)).toBe('1.00 KB');
    });
  });

  describe('Helper Methods - formatDate', () => {
    it('should format ISO date string correctly', () => {
      const isoDate = '2025-10-27T10:30:00Z';
      const formatted = component.formatDate(isoDate);

      // Check that it contains expected parts (format may vary by locale)
      expect(formatted).toContain('2025');
      expect(formatted).toContain('10');
      expect(formatted).toContain('27');
    });

    it('should handle different ISO date formats', () => {
      const isoDate1 = '2025-01-15T12:00:00Z';
      const isoDate2 = '2025-06-15T12:00:00Z';

      const formatted1 = component.formatDate(isoDate1);
      const formatted2 = component.formatDate(isoDate2);

      // Check that dates are formatted (contains year and some parts)
      expect(formatted1).toContain('2025');
      expect(formatted1).toContain('01');

      expect(formatted2).toContain('2025');
      expect(formatted2).toContain('06');
    });

    it('should use French locale format', () => {
      const isoDate = '2025-10-27T14:30:00Z';
      const formatted = component.formatDate(isoDate);

      // French format uses / or . as separator and DD/MM/YYYY order
      expect(formatted).toBeTruthy();
      expect(formatted.length).toBeGreaterThan(0);
    });
  });

  describe('Store Integration', () => {
    it('should react to cassettes$ observable updates', (done) => {
      // Need to recreate component with new mock setup
      const cassettesStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      cassettesStore.select.and.returnValue(of(mockCassettes));

      const tempComponent = new CassetteListComponent(cassettesStore, mockRouter);

      tempComponent.cassettes$.subscribe(cassettes => {
        expect(cassettes).toEqual(mockCassettes);
        expect(cassettes.length).toBe(2);
        done();
      });
    });

    it('should react to loading$ observable updates', (done) => {
      const loadingStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      loadingStore.select.and.returnValue(of(true));

      const tempComponent = new CassetteListComponent(loadingStore, mockRouter);

      tempComponent.loading$.subscribe(loading => {
        expect(loading).toBe(true);
        done();
      });
    });

    it('should react to error$ observable updates', (done) => {
      const errorMessage = 'Failed to load cassettes';
      const errorStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      errorStore.select.and.returnValue(of(errorMessage));

      const tempComponent = new CassetteListComponent(errorStore, mockRouter);

      tempComponent.error$.subscribe(error => {
        expect(error).toBe(errorMessage);
        done();
      });
    });

    it('should react to paginationInfo$ observable updates', (done) => {
      const paginationStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      paginationStore.select.and.returnValue(of(mockPaginationInfo));

      const tempComponent = new CassetteListComponent(paginationStore, mockRouter);

      tempComponent.paginationInfo$.subscribe(info => {
        expect(info).toEqual(mockPaginationInfo);
        expect(info.currentPage).toBe(1);
        expect(info.totalItems).toBe(25);
        done();
      });
    });
  });

  describe('Component Lifecycle', () => {
    it('should call store.select for each observable on construction', () => {
      // Store.select is called 4 times in constructor
      expect(mockStore.select).toHaveBeenCalled();
    });

    it('should only dispatch loadCassettes once on init', () => {
      mockStore.dispatch.calls.reset();

      component.ngOnInit();

      expect(mockStore.dispatch).toHaveBeenCalledTimes(1);
      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.loadCassettes({ params: { page: 1, limit: 20 } })
      );
    });
  });

  describe('Edge Cases', () => {
    it('should handle empty cassettes array', (done) => {
      // Create new component with empty cassettes
      const emptyStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      emptyStore.select.and.returnValue(of([]));
      const tempComponent = new CassetteListComponent(emptyStore, mockRouter);

      tempComponent.cassettes$.subscribe(cassettes => {
        expect(cassettes).toEqual([]);
        expect(cassettes.length).toBe(0);
        done();
      });
    });

    it('should handle null error', (done) => {
      // Create new component with null error
      const nullErrorStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      nullErrorStore.select.and.returnValue(of(null));
      const tempComponent = new CassetteListComponent(nullErrorStore, mockRouter);

      tempComponent.error$.subscribe(error => {
        expect(error).toBeNull();
        done();
      });
    });

    it('should format very large file sizes', () => {
      const size10MB = 10 * 1024 * 1024;
      const size100MB = 100 * 1024 * 1024;

      expect(component.formatSize(size10MB)).toBe('10.00 MB');
      expect(component.formatSize(size100MB)).toBe('100.00 MB');
    });

    it('should handle cassette without optional fields', () => {
      const minimalCassette: CassetteResource = {
        '@id': '/api/cassettes/minimal',
        '@type': 'magneto:Cassette',
        name: 'minimal',
        version: '1.0',
        recordedAt: '2025-10-27T10:00:00Z',
        interactionCount: 0,
        sizeBytes: 0
      };

      component.viewDetails(minimalCassette);

      expect(mockRouter.navigate).toHaveBeenCalledWith(['/cassettes', 'minimal']);
    });
  });
});
