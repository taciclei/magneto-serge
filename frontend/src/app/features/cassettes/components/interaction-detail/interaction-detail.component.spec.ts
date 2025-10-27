import { ComponentFixture, TestBed, fakeAsync, tick, flush } from '@angular/core/testing';
import { ActivatedRoute, Router } from '@angular/router';
import { MatSnackBar } from '@angular/material/snack-bar';
import { of, throwError } from 'rxjs';

import { InteractionDetailComponent } from './interaction-detail.component';
import { AlcaeusService } from '../../../../core/services/alcaeus.service';
import {
  HttpInteractionResource,
  WebSocketInteractionResource,
} from '../../../../core/models/interaction.model';
import { NoopAnimationsModule } from '@angular/platform-browser/animations';

describe('InteractionDetailComponent', () => {
  let component: InteractionDetailComponent;
  let fixture: ComponentFixture<InteractionDetailComponent>;
  let mockActivatedRoute: jasmine.SpyObj<ActivatedRoute>;
  let mockRouter: jasmine.SpyObj<Router>;
  let mockAlcaeusService: jasmine.SpyObj<AlcaeusService>;
  let mockSnackBar: jasmine.SpyObj<MatSnackBar>;

  // Mock HTTP Interaction
  const mockHttpInteraction: HttpInteractionResource = {
    '@id': '/api/cassettes/test-cassette/interactions/1',
    '@type': 'Interaction',
    kind: 'Http',
    request: {
      method: 'GET',
      url: 'https://api.example.com/users',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer token123'
      },
      body: undefined
    },
    response: {
      status: 200,
      headers: {
        'Content-Type': 'application/json'
      },
      body: '{"users": [{"id": 1, "name": "John"}]}',
      hasTemplates: false
    }
  };

  // Mock WebSocket Interaction
  const mockWsInteraction: WebSocketInteractionResource = {
    '@id': '/api/cassettes/test-cassette/interactions/2',
    '@type': 'Interaction',
    kind: 'WebSocket',
    url: 'wss://api.example.com/ws',
    messages: [
      {
        direction: 'Sent',
        timestampMs: 0,
        msgType: 'Text',
        data: '{"action": "subscribe"}'
      },
      {
        direction: 'Received',
        timestampMs: 100,
        msgType: 'Text',
        data: '{"status": "subscribed"}'
      },
      {
        direction: 'Received',
        timestampMs: 200,
        msgType: 'Text',
        data: '{"data": "message1"}'
      }
    ]
  };

  beforeEach(async () => {
    // Create spies
    mockActivatedRoute = jasmine.createSpyObj('ActivatedRoute', [], {
      paramMap: of(new Map([
        ['cassetteName', 'test-cassette'],
        ['interactionId', '1']
      ]))
    });

    // Mock paramMap with proper structure
    Object.defineProperty(mockActivatedRoute, 'paramMap', {
      get: () => of({
        get: (key: string) => {
          if (key === 'cassetteName') return 'test-cassette';
          if (key === 'interactionId') return '1';
          return null;
        }
      })
    });

    mockRouter = jasmine.createSpyObj('Router', ['navigate']);
    mockAlcaeusService = jasmine.createSpyObj('AlcaeusService', ['loadResource']);
    mockSnackBar = jasmine.createSpyObj('MatSnackBar', ['open']);

    await TestBed.configureTestingModule({
      imports: [
        InteractionDetailComponent,
        NoopAnimationsModule
      ],
      providers: [
        { provide: ActivatedRoute, useValue: mockActivatedRoute },
        { provide: Router, useValue: mockRouter },
        { provide: AlcaeusService, useValue: mockAlcaeusService },
        { provide: MatSnackBar, useValue: mockSnackBar }
      ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(InteractionDetailComponent);
    component = fixture.componentInstance;
  });

  afterEach(() => {
    fixture.destroy();
  });

  describe('Component Initialization', () => {
    it('should create', () => {
      expect(component).toBeTruthy();
    });

    it('should initialize with loading state', () => {
      expect(component.loading).toBe(true);
      expect(component.error).toBeNull();
      expect(component.interaction).toBeNull();
    });

    it('should extract route parameters on init', () => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockHttpInteraction as any));

      fixture.detectChanges(); // triggers ngOnInit

      expect(component.cassetteName).toBe('test-cassette');
      expect(component.interactionId).toBe('1');
    });

    it('should load interaction on init with valid params', async () => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockHttpInteraction as any));

      fixture.detectChanges();
      await fixture.whenStable();

      expect(mockAlcaeusService.loadResource).toHaveBeenCalledWith(
        '/api/cassettes/test-cassette/interactions/1'
      );
      expect(component.interaction).toEqual(mockHttpInteraction);
      expect(component.loading).toBe(false);
    });

    it('should handle missing route parameters', () => {
      // Override paramMap to return null values
      Object.defineProperty(mockActivatedRoute, 'paramMap', {
        get: () => of({
          get: () => null
        })
      });

      fixture.detectChanges();

      expect(component.error).toBe('Invalid route parameters');
      expect(component.loading).toBe(false);
      expect(mockAlcaeusService.loadResource).not.toHaveBeenCalled();
    });
  });

  describe('loadInteraction()', () => {
    beforeEach(() => {
      component.cassetteName = 'test-cassette';
      component.interactionId = '1';
    });

    it('should load HTTP interaction successfully', async () => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockHttpInteraction as any));

      await component.loadInteraction();

      expect(component.interaction).toEqual(mockHttpInteraction);
      expect(component.loading).toBe(false);
      expect(component.error).toBeNull();
    });

    it('should load WebSocket interaction successfully', async () => {
      mockAlcaeusService.loadResource.and.returnValue(of(mockWsInteraction as any));

      await component.loadInteraction();

      expect(component.interaction).toEqual(mockWsInteraction);
      expect(component.loading).toBe(false);
      expect(component.error).toBeNull();
    });

    it('should handle load error', async () => {
      const errorMessage = 'Network error';
      mockAlcaeusService.loadResource.and.returnValue(
        throwError(() => new Error(errorMessage))
      );

      await component.loadInteraction();
      await fixture.whenStable();
      fixture.detectChanges();

      expect(component.error).toBe(errorMessage);
      expect(component.loading).toBe(false);
      expect(component.interaction).toBeNull();
      expect(mockSnackBar.open).toHaveBeenCalledWith(
        `Error: ${errorMessage}`,
        'Close',
        jasmine.objectContaining({ duration: 5000 })
      );
    });

    it('should handle error without message', async () => {
      mockAlcaeusService.loadResource.and.returnValue(
        throwError(() => ({}))
      );

      await component.loadInteraction();
      await fixture.whenStable();
      fixture.detectChanges();

      expect(component.error).toBe('Failed to load interaction');
    });
  });

  describe('Type Guards', () => {
    it('isHttp should return true for HTTP interaction', () => {
      component.interaction = mockHttpInteraction;

      expect(component.isHttp).toBe(true);
      expect(component.isWebSocket).toBe(false);
    });

    it('isWebSocket should return true for WebSocket interaction', () => {
      component.interaction = mockWsInteraction;

      expect(component.isHttp).toBe(false);
      expect(component.isWebSocket).toBe(true);
    });

    it('should return false when interaction is null', () => {
      component.interaction = null;

      expect(component.isHttp).toBe(false);
      expect(component.isWebSocket).toBe(false);
    });

    it('httpInteraction getter should return interaction when HTTP', () => {
      component.interaction = mockHttpInteraction;

      expect(component.httpInteraction).toEqual(mockHttpInteraction);
    });

    it('httpInteraction getter should return null when WebSocket', () => {
      component.interaction = mockWsInteraction;

      expect(component.httpInteraction).toBeNull();
    });

    it('wsInteraction getter should return interaction when WebSocket', () => {
      component.interaction = mockWsInteraction;

      expect(component.wsInteraction).toEqual(mockWsInteraction);
    });

    it('wsInteraction getter should return null when HTTP', () => {
      component.interaction = mockHttpInteraction;

      expect(component.wsInteraction).toBeNull();
    });
  });

  describe('WebSocket Message Counting', () => {
    beforeEach(() => {
      component.interaction = mockWsInteraction;
    });

    it('sentMessagesCount should count sent messages', () => {
      expect(component.sentMessagesCount).toBe(1);
    });

    it('receivedMessagesCount should count received messages', () => {
      expect(component.receivedMessagesCount).toBe(2);
    });

    it('should return 0 when not WebSocket interaction', () => {
      component.interaction = mockHttpInteraction;

      expect(component.sentMessagesCount).toBe(0);
      expect(component.receivedMessagesCount).toBe(0);
    });

    it('should return 0 when interaction is null', () => {
      component.interaction = null;

      expect(component.sentMessagesCount).toBe(0);
      expect(component.receivedMessagesCount).toBe(0);
    });
  });

  describe('Message Direction Helpers', () => {
    it('isMessageSent should return true for Sent', () => {
      expect(component.isMessageSent('Sent')).toBe(true);
      expect(component.isMessageSent('Received')).toBe(false);
    });

    it('isMessageReceived should return true for Received', () => {
      expect(component.isMessageReceived('Received')).toBe(true);
      expect(component.isMessageReceived('Sent')).toBe(false);
    });

    it('getMessageDirectionColor should return correct color', () => {
      expect(component.getMessageDirectionColor('Sent')).toBe('primary');
      expect(component.getMessageDirectionColor('Received')).toBe('accent');
    });

    it('getMessageDirectionIcon should return correct icon', () => {
      expect(component.getMessageDirectionIcon('Sent')).toBe('arrow_upward');
      expect(component.getMessageDirectionIcon('Received')).toBe('arrow_downward');
    });
  });

  describe('Status Helpers', () => {
    it('getStatusText should return correct text for known statuses', () => {
      expect(component.getStatusText(200)).toBe('OK');
      expect(component.getStatusText(404)).toBe('Not Found');
      expect(component.getStatusText(500)).toBe('Internal Server Error');
    });

    it('getStatusText should return Unknown for unknown statuses', () => {
      expect(component.getStatusText(999)).toBe('Unknown');
    });
  });

  describe('JSON Formatting', () => {
    it('formatJson should format valid JSON', () => {
      const json = '{"name":"John","age":30}';
      const result = component.formatJson(json);

      expect(result).toContain('"name": "John"');
      expect(result).toContain('"age": 30');
    });

    it('formatJson should return as-is for non-JSON', () => {
      const text = 'plain text';

      expect(component.formatJson(text)).toBe(text);
    });

    it('formatJson should handle undefined body', () => {
      expect(component.formatJson(undefined)).toBe('No body');
    });

    it('isJsonBody should return true for valid JSON', () => {
      expect(component.isJsonBody('{"valid": true}')).toBe(true);
    });

    it('isJsonBody should return false for invalid JSON', () => {
      expect(component.isJsonBody('not json')).toBe(false);
    });

    it('isJsonBody should return false for undefined', () => {
      expect(component.isJsonBody(undefined)).toBe(false);
    });
  });

  describe('Helper Methods', () => {
    it('isEmpty should return true for 0', () => {
      expect(component.isEmpty(0)).toBe(true);
    });

    it('isEmpty should return false for non-zero', () => {
      expect(component.isEmpty(1)).toBe(false);
      expect(component.isEmpty(10)).toBe(false);
    });

    it('getMethodColor should delegate to model helper', () => {
      // Test some known method colors
      expect(component.getMethodColor('GET')).toBeTruthy();
      expect(component.getMethodColor('POST')).toBeTruthy();
    });

    it('getStatusColor should delegate to model helper', () => {
      // Test some known status colors
      expect(component.getStatusColor(200)).toBeTruthy();
      expect(component.getStatusColor(404)).toBeTruthy();
    });
  });

  describe('Component Lifecycle', () => {
    it('should unsubscribe on destroy', () => {
      const destroySpy = spyOn(component['destroy$'], 'next');
      const completeSpy = spyOn(component['destroy$'], 'complete');

      component.ngOnDestroy();

      expect(destroySpy).toHaveBeenCalled();
      expect(completeSpy).toHaveBeenCalled();
    });
  });
});
