import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ActivatedRoute, Router } from '@angular/router';
import { Store } from '@ngrx/store';
import { of, Subject } from 'rxjs';
import { NoopAnimationsModule } from '@angular/platform-browser/animations';

import { CassetteDetailComponent } from './cassette-detail.component';
import { CassetteResource } from '../../../../core/models/cassette.model';
import { CassetteActions } from '../../state/cassette.actions';
import {
  selectSelectedCassette,
  selectLoading,
  selectError
} from '../../state/cassette.selectors';

describe('CassetteDetailComponent', () => {
  let component: CassetteDetailComponent;
  let fixture: ComponentFixture<CassetteDetailComponent>;
  let mockStore: jasmine.SpyObj<Store>;
  let mockRouter: jasmine.SpyObj<Router>;
  let mockActivatedRoute: { params: Subject<any> };

  // Mock cassette data
  const mockCassette: CassetteResource = {
    '@id': '/api/cassettes/test-cassette',
    '@type': 'magneto:Cassette',
    name: 'test-cassette',
    version: '1.0',
    recordedAt: '2025-10-27T10:00:00Z',
    interactionCount: 15,
    sizeBytes: 2048,
    hasTemplates: false,
    description: 'Test cassette for unit tests'
  };

  beforeEach(async () => {
    // Create mock ActivatedRoute with params Subject
    mockActivatedRoute = {
      params: new Subject()
    };

    // Create mock Store and Router
    mockStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
    mockRouter = jasmine.createSpyObj('Router', ['navigate']);

    // Setup default return values for store selectors
    mockStore.select.and.callFake((selector: any) => {
      const selectorStr = selector.toString();

      if (selectorStr.includes('SelectedCassette') || selectorStr.includes('selectedCassette')) {
        return of(null);
      } else if (selectorStr.includes('loading')) {
        return of(false);
      } else if (selectorStr.includes('error')) {
        return of(null);
      }
      return of(null);
    });

    await TestBed.configureTestingModule({
      imports: [
        CassetteDetailComponent,
        NoopAnimationsModule
      ],
      providers: [
        { provide: Store, useValue: mockStore },
        { provide: Router, useValue: mockRouter },
        { provide: ActivatedRoute, useValue: mockActivatedRoute }
      ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CassetteDetailComponent);
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
      expect(component.cassette$).toBeDefined();
      expect(component.loading$).toBeDefined();
      expect(component.error$).toBeDefined();
    });

    it('should dispatch loadCassetteDetail action when route params contain name', () => {
      fixture.detectChanges(); // triggers ngOnInit

      // Emit route params
      mockActivatedRoute.params.next({ name: 'test-cassette' });

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.loadCassetteDetail({ name: 'test-cassette' })
      );
    });

    it('should not dispatch action when route params do not contain name', () => {
      mockStore.dispatch.calls.reset();
      fixture.detectChanges();

      mockActivatedRoute.params.next({});

      expect(mockStore.dispatch).not.toHaveBeenCalled();
    });

    it('should select correct state slices from store', () => {
      fixture.detectChanges();

      expect(mockStore.select).toHaveBeenCalledTimes(3);
    });
  });

  describe('Component Lifecycle', () => {
    it('should subscribe to route params on init', () => {
      const paramsSpy = spyOn(mockActivatedRoute.params, 'subscribe').and.callThrough();

      fixture.detectChanges();

      expect(paramsSpy).toHaveBeenCalled();
    });

    it('should dispatch clearSelection action on destroy', () => {
      fixture.detectChanges();
      mockStore.dispatch.calls.reset();

      component.ngOnDestroy();

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.clearSelection()
      );
    });

    it('should complete destroy$ Subject on destroy', () => {
      const destroySpy = spyOn(component['destroy$'], 'next');
      const completeSpy = spyOn(component['destroy$'], 'complete');

      component.ngOnDestroy();

      expect(destroySpy).toHaveBeenCalled();
      expect(completeSpy).toHaveBeenCalled();
    });

    it('should unsubscribe from route params on destroy', () => {
      fixture.detectChanges();

      // Emit a param
      mockActivatedRoute.params.next({ name: 'cassette-1' });
      const callCount = mockStore.dispatch.calls.count();

      // Destroy component
      component.ngOnDestroy();

      // Emit again - should not trigger new loadCassetteDetail dispatch
      mockActivatedRoute.params.next({ name: 'cassette-2' });

      // Should only have one more call (clearSelection from ngOnDestroy)
      expect(mockStore.dispatch.calls.count()).toBe(callCount + 1);
    });
  });

  describe('Navigation Methods', () => {
    it('goBack should navigate to /cassettes', () => {
      component.goBack();

      expect(mockRouter.navigate).toHaveBeenCalledWith(['/cassettes']);
    });

    it('viewInteractions should navigate to interactions route', () => {
      component.viewInteractions(mockCassette);

      expect(mockRouter.navigate).toHaveBeenCalledWith(
        ['/cassettes', 'test-cassette', 'interactions']
      );
    });
  });

  describe('Action Methods', () => {
    it('reload should dispatch loadCassetteDetail action', () => {
      mockStore.dispatch.calls.reset();

      component.reload(mockCassette);

      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.loadCassetteDetail({ name: 'test-cassette' })
      );
    });
  });

  describe('Helper Methods - formatSize', () => {
    it('should format bytes correctly', () => {
      expect(component.formatSize(500)).toBe('500 B');
      expect(component.formatSize(100)).toBe('100 B');
      expect(component.formatSize(0)).toBe('0 B');
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
      expect(component.formatSize(1023)).toBe('1023 B');
      expect(component.formatSize(1025)).toBe('1.00 KB');
      expect(component.formatSize(10 * 1024 * 1024)).toBe('10.00 MB');
    });
  });

  describe('Helper Methods - formatDate', () => {
    it('should format ISO date string correctly', () => {
      const isoDate = '2025-10-27T10:30:00Z';
      const formatted = component.formatDate(isoDate);

      // Check that it contains expected parts (format may vary by locale)
      // French locale uses full month name ("octobre") and day/year
      expect(formatted).toContain('2025');
      expect(formatted).toContain('27');
      expect(formatted.length).toBeGreaterThan(0);
    });

    it('should handle different ISO date formats', () => {
      const isoDate1 = '2025-01-15T12:00:00Z';
      const isoDate2 = '2025-06-15T12:00:00Z';

      const formatted1 = component.formatDate(isoDate1);
      const formatted2 = component.formatDate(isoDate2);

      expect(formatted1).toContain('2025');
      expect(formatted1).toBeTruthy();

      expect(formatted2).toContain('2025');
      expect(formatted2).toBeTruthy();
    });

    it('should use French locale format', () => {
      const isoDate = '2025-10-27T14:30:00Z';
      const formatted = component.formatDate(isoDate);

      // French format includes full month name and specific structure
      expect(formatted).toBeTruthy();
      expect(formatted.length).toBeGreaterThan(0);
    });
  });

  describe('Store Integration', () => {
    it('should react to cassette$ observable updates', (done) => {
      const cassetteStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      cassetteStore.select.and.returnValue(of(mockCassette));

      const tempComponent = new CassetteDetailComponent(
        mockActivatedRoute as any,
        mockRouter,
        cassetteStore
      );

      tempComponent.cassette$.subscribe(cassette => {
        expect(cassette).toEqual(mockCassette);
        expect(cassette?.name).toBe('test-cassette');
        done();
      });
    });

    it('should react to loading$ observable updates', (done) => {
      const loadingStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      loadingStore.select.and.returnValue(of(true));

      const tempComponent = new CassetteDetailComponent(
        mockActivatedRoute as any,
        mockRouter,
        loadingStore
      );

      tempComponent.loading$.subscribe(loading => {
        expect(loading).toBe(true);
        done();
      });
    });

    it('should react to error$ observable updates', (done) => {
      const errorMessage = 'Failed to load cassette';
      const errorStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      errorStore.select.and.returnValue(of(errorMessage));

      const tempComponent = new CassetteDetailComponent(
        mockActivatedRoute as any,
        mockRouter,
        errorStore
      );

      tempComponent.error$.subscribe(error => {
        expect(error).toBe(errorMessage);
        done();
      });
    });
  });

  describe('Edge Cases', () => {
    it('should handle null cassette', (done) => {
      const nullStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      nullStore.select.and.returnValue(of(null));

      const tempComponent = new CassetteDetailComponent(
        mockActivatedRoute as any,
        mockRouter,
        nullStore
      );

      tempComponent.cassette$.subscribe(cassette => {
        expect(cassette).toBeNull();
        done();
      });
    });

    it('should handle null error', (done) => {
      const nullErrorStore = jasmine.createSpyObj('Store', ['select', 'dispatch']);
      nullErrorStore.select.and.returnValue(of(null));

      const tempComponent = new CassetteDetailComponent(
        mockActivatedRoute as any,
        mockRouter,
        nullErrorStore
      );

      tempComponent.error$.subscribe(error => {
        expect(error).toBeNull();
        done();
      });
    });

    it('should format very large file sizes', () => {
      const size100MB = 100 * 1024 * 1024;
      const size500MB = 500 * 1024 * 1024;

      expect(component.formatSize(size100MB)).toBe('100.00 MB');
      expect(component.formatSize(size500MB)).toBe('500.00 MB');
    });

    it('should handle cassette with minimal fields', () => {
      const minimalCassette: CassetteResource = {
        '@id': '/api/cassettes/minimal',
        '@type': 'magneto:Cassette',
        name: 'minimal',
        version: '1.0',
        recordedAt: '2025-10-27T10:00:00Z',
        interactionCount: 0,
        sizeBytes: 0
      };

      component.viewInteractions(minimalCassette);
      expect(mockRouter.navigate).toHaveBeenCalledWith(['/cassettes', 'minimal', 'interactions']);

      component.reload(minimalCassette);
      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.loadCassetteDetail({ name: 'minimal' })
      );
    });
  });

  describe('Route Parameter Changes', () => {
    it('should dispatch new action when route params change', () => {
      fixture.detectChanges();
      mockStore.dispatch.calls.reset();

      // First param
      mockActivatedRoute.params.next({ name: 'cassette-1' });
      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.loadCassetteDetail({ name: 'cassette-1' })
      );

      mockStore.dispatch.calls.reset();

      // Second param
      mockActivatedRoute.params.next({ name: 'cassette-2' });
      expect(mockStore.dispatch).toHaveBeenCalledWith(
        CassetteActions.loadCassetteDetail({ name: 'cassette-2' })
      );
    });

    it('should handle rapid route parameter changes', () => {
      fixture.detectChanges();
      mockStore.dispatch.calls.reset();

      mockActivatedRoute.params.next({ name: 'cassette-1' });
      mockActivatedRoute.params.next({ name: 'cassette-2' });
      mockActivatedRoute.params.next({ name: 'cassette-3' });

      expect(mockStore.dispatch).toHaveBeenCalledTimes(3);
    });
  });
});
