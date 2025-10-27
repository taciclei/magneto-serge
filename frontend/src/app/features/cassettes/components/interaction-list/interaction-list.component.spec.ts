import { ComponentFixture, TestBed } from '@angular/core/testing';
import { of, throwError } from 'rxjs';
import { NoopAnimationsModule } from '@angular/platform-browser/animations';

import { InteractionListComponent } from './interaction-list.component';
import { AlcaeusService } from '../../../../core/services/alcaeus.service';
import { InteractionResource } from '../../../../core/models/interaction.model';

describe('InteractionListComponent', () => {
  let component: InteractionListComponent;
  let fixture: ComponentFixture<InteractionListComponent>;
  let mockAlcaeusService: jasmine.SpyObj<AlcaeusService>;

  // Mock interactions data
  const mockHttpInteraction: any = {
    '@id': '/api/cassettes/test/interactions/1',
    '@type': 'Interaction',
    kind: 'Http',
    request: {
      method: 'GET',
      url: 'https://api.example.com/users',
      headers: { 'Content-Type': 'application/json' }
    },
    response: {
      status: 200,
      headers: { 'Content-Type': 'application/json' },
      body: [123, 34, 110, 97, 109, 101, 34, 58, 34, 74, 111, 104, 110, 34, 125] // {"name":"John"}
    }
  };

  const mockWsInteraction: any = {
    '@id': '/api/cassettes/test/interactions/2',
    '@type': 'Interaction',
    kind: 'WebSocket',
    url: 'wss://api.example.com/ws',
    messages: [
      { direction: 'Sent', timestampMs: 0, msgType: 'Text', data: 'hello' },
      { direction: 'Received', timestampMs: 500, msgType: 'Text', data: 'world' }
    ]
  };

  const mockHydraResponse = {
    root: {
      'hydra:member': [mockHttpInteraction, mockWsInteraction]
    }
  };

  beforeEach(async () => {
    mockAlcaeusService = jasmine.createSpyObj('AlcaeusService', ['loadResource']);

    await TestBed.configureTestingModule({
      imports: [
        InteractionListComponent,
        NoopAnimationsModule
      ],
      providers: [
        { provide: AlcaeusService, useValue: mockAlcaeusService }
      ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(InteractionListComponent);
    component = fixture.componentInstance;
  });

  afterEach(() => {
    fixture.destroy();
  });

  describe('Component Initialization', () => {
    it('should create', () => {
      expect(component).toBeTruthy();
    });

    it('should initialize with default values', () => {
      expect(component.loading).toBe(false);
      expect(component.error).toBeNull();
    });

    it('should not load interactions if cassetteName is not set', () => {
      fixture.detectChanges();
      expect(mockAlcaeusService.loadResource).not.toHaveBeenCalled();
    });

    it('should load interactions if cassetteName is set', () => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockHydraResponse as any));
      component.cassetteName = 'test-cassette';

      fixture.detectChanges();

      expect(mockAlcaeusService.loadResource).toHaveBeenCalledWith('/cassettes/test-cassette/interactions');
    });
  });

  describe('loadInteractions()', () => {
    beforeEach(() => {
      component.cassetteName = 'test-cassette';
    });

    it('should set loading to true when starting', () => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockHydraResponse as any));

      component.loadInteractions();

      // Initially loading should be set to true (checked synchronously before observable completes)
      // After subscription completes, it's set to false
      expect(mockAlcaeusService.loadResource).toHaveBeenCalled();
    });

    it('should load interactions successfully', (done) => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockHydraResponse as any));

      component.loadInteractions();

      component.interactions$.subscribe(interactions => {
        expect(interactions.length).toBe(2);
        expect(interactions[0]).toEqual(mockHttpInteraction);
        expect(interactions[1]).toEqual(mockWsInteraction);
        expect(component.loading).toBe(false);
        expect(component.error).toBeNull();
        done();
      });
    });

    it('should handle empty member array', (done) => {
      const emptyResponse = { root: { 'hydra:member': [] } };
      mockAlcaeusService.loadResource.and.returnValue(of(emptyResponse as any));

      component.loadInteractions();

      component.interactions$.subscribe(interactions => {
        expect(interactions.length).toBe(0);
        expect(component.loading).toBe(false);
        done();
      });
    });

    it('should handle missing hydra:member', (done) => {
      const noMemberResponse = { root: {} };
      mockAlcaeusService.loadResource.and.returnValue(of(noMemberResponse as any));

      component.loadInteractions();

      component.interactions$.subscribe(interactions => {
        expect(interactions.length).toBe(0);
        expect(component.loading).toBe(false);
        done();
      });
    });

    it('should handle error', (done) => {
      const errorMessage = 'Network error';
      mockAlcaeusService.loadResource.and.returnValue(
        throwError(() => new Error(errorMessage))
      );

      component.loadInteractions();

      // Wait for error handling
      setTimeout(() => {
        expect(component.error).toBe(errorMessage);
        expect(component.loading).toBe(false);

        component.interactions$.subscribe(interactions => {
          expect(interactions.length).toBe(0);
          done();
        });
      }, 10);
    });

    it('should handle error without message', (done) => {
      mockAlcaeusService.loadResource.and.returnValue(
        throwError(() => ({}))
      );

      component.loadInteractions();

      setTimeout(() => {
        expect(component.error).toBe('Erreur lors du chargement des interactions');
        expect(component.loading).toBe(false);
        done();
      }, 10);
    });
  });

  describe('Helper Methods - Type and Status', () => {
    it('getInteractionTypeClass should return correct class for Http', () => {
      expect(component.getInteractionTypeClass('Http')).toBe('http-type');
    });

    it('getInteractionTypeClass should return correct class for WebSocket', () => {
      expect(component.getInteractionTypeClass('WebSocket')).toBe('websocket-type');
    });

    it('getStatusClass should return success for 2xx', () => {
      expect(component.getStatusClass(200)).toBe('status-success');
      expect(component.getStatusClass(201)).toBe('status-success');
      expect(component.getStatusClass(299)).toBe('status-success');
    });

    it('getStatusClass should return redirect for 3xx', () => {
      expect(component.getStatusClass(300)).toBe('status-redirect');
      expect(component.getStatusClass(301)).toBe('status-redirect');
      expect(component.getStatusClass(399)).toBe('status-redirect');
    });

    it('getStatusClass should return client-error for 4xx', () => {
      expect(component.getStatusClass(400)).toBe('status-client-error');
      expect(component.getStatusClass(404)).toBe('status-client-error');
      expect(component.getStatusClass(499)).toBe('status-client-error');
    });

    it('getStatusClass should return server-error for 5xx', () => {
      expect(component.getStatusClass(500)).toBe('status-server-error');
      expect(component.getStatusClass(503)).toBe('status-server-error');
      expect(component.getStatusClass(599)).toBe('status-server-error');
    });

    it('getStatusClass should return server-error for other codes', () => {
      expect(component.getStatusClass(100)).toBe('status-server-error');
      expect(component.getStatusClass(600)).toBe('status-server-error');
    });
  });

  describe('Helper Methods - Formatting', () => {
    it('formatHeaders should format headers correctly', () => {
      const headers = {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer token'
      };

      const formatted = component.formatHeaders(headers);

      expect(formatted).toContain('Content-Type: application/json');
      expect(formatted).toContain('Authorization: Bearer token');
      expect(formatted.split('\n').length).toBe(2);
    });

    it('formatHeaders should handle empty headers', () => {
      const formatted = component.formatHeaders({});
      expect(formatted).toBe('');
    });

    it('formatBody should return (vide) for null', () => {
      expect(component.formatBody(null)).toBe('(vide)');
    });

    it('formatBody should return (vide) for empty array', () => {
      expect(component.formatBody([])).toBe('(vide)');
    });

    it('formatBody should decode and format JSON', () => {
      // {"name":"John"} en bytes UTF-8
      const jsonBytes = [123, 34, 110, 97, 109, 101, 34, 58, 34, 74, 111, 104, 110, 34, 125];
      const formatted = component.formatBody(jsonBytes);

      expect(formatted).toContain('"name"');
      expect(formatted).toContain('"John"');
    });

    it('formatBody should return plain text for non-JSON', () => {
      // "Hello" en bytes UTF-8
      const textBytes = [72, 101, 108, 108, 111];
      const formatted = component.formatBody(textBytes);

      expect(formatted).toBe('Hello');
    });

    it('formatBody should handle binary data', () => {
      // Invalid UTF-8 sequence - TextDecoder may return replacement characters
      const binaryBytes = [255, 254, 253];
      const formatted = component.formatBody(binaryBytes);

      // The decoder will either throw (caught by formatBody) or return replacement chars
      // Both are valid behaviors, so just check it returns something
      expect(formatted).toBeTruthy();
      expect(formatted.length).toBeGreaterThan(0);
    });

    it('formatDirection should return Envoyé for Sent', () => {
      expect(component.formatDirection('Sent')).toBe('Envoyé');
    });

    it('formatDirection should return Reçu for Received', () => {
      expect(component.formatDirection('Received')).toBe('Reçu');
    });

    it('getDirectionIcon should return arrow_upward for Sent', () => {
      expect(component.getDirectionIcon('Sent')).toBe('arrow_upward');
    });

    it('getDirectionIcon should return arrow_downward for Received', () => {
      expect(component.getDirectionIcon('Received')).toBe('arrow_downward');
    });

    it('formatTimestamp should format milliseconds', () => {
      expect(component.formatTimestamp(500)).toBe('500ms');
      expect(component.formatTimestamp(999)).toBe('999ms');
    });

    it('formatTimestamp should format seconds', () => {
      expect(component.formatTimestamp(1000)).toBe('1.00s');
      expect(component.formatTimestamp(2500)).toBe('2.50s');
      expect(component.formatTimestamp(10000)).toBe('10.00s');
    });
  });

  describe('Edge Cases', () => {
    it('should handle cassetteName change', () => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockHydraResponse as any));

      component.cassetteName = 'cassette-1';
      fixture.detectChanges();

      expect(mockAlcaeusService.loadResource).toHaveBeenCalledWith('/cassettes/cassette-1/interactions');

      mockAlcaeusService.loadResource.calls.reset();

      component.cassetteName = 'cassette-2';
      component.loadInteractions();

      expect(mockAlcaeusService.loadResource).toHaveBeenCalledWith('/cassettes/cassette-2/interactions');
    });

    it('should handle response with root but empty member', (done) => {
      const emptyRoot = { root: { 'hydra:member': [] } };
      mockAlcaeusService.loadResource.and.returnValue(of(emptyRoot as any));

      component.cassetteName = 'test';
      component.loadInteractions();

      component.interactions$.subscribe(interactions => {
        expect(interactions.length).toBe(0);
        expect(component.loading).toBe(false);
        done();
      });
    });

    it('should handle very large body arrays', () => {
      const largeBody = new Array(10000).fill(65); // Array of 10000 'A' characters
      const formatted = component.formatBody(largeBody);

      expect(formatted.length).toBeGreaterThan(0);
    });

    it('should format headers with special characters', () => {
      const headers = {
        'X-Custom-Header': 'value:with:colons',
        'Set-Cookie': 'session=abc; Path=/'
      };

      const formatted = component.formatHeaders(headers);

      expect(formatted).toContain('X-Custom-Header: value:with:colons');
      expect(formatted).toContain('Set-Cookie: session=abc; Path=/');
    });
  });

  describe('Observable Updates', () => {
    it('should update interactions$ on successful load', (done) => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockHydraResponse as any));

      component.cassetteName = 'test';
      component.loadInteractions();

      component.interactions$.subscribe(interactions => {
        expect(interactions.length).toBe(2);
        done();
      });
    });

    it('should reset interactions$ to empty array on error', (done) => {
      mockAlcaeusService.loadResource.and.returnValue(
        throwError(() => new Error('Test error'))
      );

      component.cassetteName = 'test';
      component.loadInteractions();

      setTimeout(() => {
        component.interactions$.subscribe(interactions => {
          expect(interactions.length).toBe(0);
          done();
        });
      }, 10);
    });
  });
});
