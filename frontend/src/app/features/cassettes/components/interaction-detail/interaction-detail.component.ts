import { Component, OnInit, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, Router, RouterModule } from '@angular/router';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatChipsModule } from '@angular/material/chips';
import { MatDividerModule } from '@angular/material/divider';
import { MatTabsModule } from '@angular/material/tabs';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatSnackBar, MatSnackBarModule } from '@angular/material/snack-bar';
import { Store } from '@ngrx/store';
import { Observable, Subject, takeUntil } from 'rxjs';

import { AlcaeusService } from '../../../../core/services/alcaeus.service';
import {
  InteractionResource,
  HttpInteractionResource,
  WebSocketInteractionResource,
  isHttpInteraction,
  isWebSocketInteraction,
  getMethodColor,
  getStatusColor
} from '../../../../core/models/interaction.model';

/**
 * Composant de détail d'une interaction unique
 *
 * Affiche les détails complets d'une interaction HTTP ou WebSocket:
 * - HTTP: request/response avec headers et body
 * - WebSocket: timeline des messages avec direction et timestamps
 *
 * Route: /cassettes/:cassetteName/interactions/:interactionId
 */
@Component({
  selector: 'app-interaction-detail',
  standalone: true,
  imports: [
    CommonModule,
    RouterModule,
    MatCardModule,
    MatButtonModule,
    MatIconModule,
    MatChipsModule,
    MatDividerModule,
    MatTabsModule,
    MatExpansionModule,
    MatProgressSpinnerModule,
    MatSnackBarModule
  ],
  templateUrl: './interaction-detail.component.html',
  styleUrl: './interaction-detail.component.scss'
})
export class InteractionDetailComponent implements OnInit, OnDestroy {
  cassetteName: string = '';
  interactionId: string = '';
  interaction: InteractionResource | null = null;
  loading = true;
  error: string | null = null;

  // Expose Object to template
  Object = Object;

  private destroy$ = new Subject<void>();

  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private alcaeus: AlcaeusService,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit(): void {
    // Récupérer les paramètres de route
    this.route.paramMap.pipe(takeUntil(this.destroy$)).subscribe(params => {
      this.cassetteName = params.get('cassetteName') || '';
      this.interactionId = params.get('interactionId') || '';

      if (this.cassetteName && this.interactionId) {
        this.loadInteraction();
      } else {
        this.error = 'Invalid route parameters';
        this.loading = false;
      }
    });
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }

  /**
   * Charge l'interaction depuis l'API Hydra
   */
  async loadInteraction(): Promise<void> {
    this.loading = true;
    this.error = null;

    try {
      const url = `/api/cassettes/${this.cassetteName}/interactions/${this.interactionId}`;
      const response = await this.alcaeus.loadResource<any>(url);

      // Extract the actual resource from the Hydra response
      this.interaction = response as unknown as InteractionResource;
      this.loading = false;
    } catch (err: any) {
      console.error('Failed to load interaction:', err);
      this.error = err.message || 'Failed to load interaction';
      this.loading = false;

      this.snackBar.open(`Error: ${this.error}`, 'Close', {
        duration: 5000,
        panelClass: ['error-snackbar']
      });
    }
  }

  /**
   * Type guards pour le template
   */
  get isHttp(): boolean {
    return this.interaction ? isHttpInteraction(this.interaction) : false;
  }

  get isWebSocket(): boolean {
    return this.interaction ? isWebSocketInteraction(this.interaction) : false;
  }

  get httpInteraction(): HttpInteractionResource | null {
    return this.isHttp ? (this.interaction as HttpInteractionResource) : null;
  }

  get wsInteraction(): WebSocketInteractionResource | null {
    return this.isWebSocket ? (this.interaction as WebSocketInteractionResource) : null;
  }

  /**
   * Compte les messages WebSocket envoyés
   */
  get sentMessagesCount(): number {
    if (!this.wsInteraction) return 0;
    return this.wsInteraction.messages.filter(m => m.direction === 'Sent').length;
  }

  /**
   * Compte les messages WebSocket reçus
   */
  get receivedMessagesCount(): number {
    if (!this.wsInteraction) return 0;
    return this.wsInteraction.messages.filter(m => m.direction === 'Received').length;
  }

  /**
   * Vérifie si un message est envoyé
   */
  isMessageSent(direction: string): boolean {
    return direction === 'Sent';
  }

  /**
   * Vérifie si un message est reçu
   */
  isMessageReceived(direction: string): boolean {
    return direction === 'Received';
  }

  /**
   * Retourne la couleur pour la direction du message
   */
  getMessageDirectionColor(direction: string): string {
    return direction === 'Sent' ? 'primary' : 'accent';
  }

  /**
   * Retourne l'icône pour la direction du message
   */
  getMessageDirectionIcon(direction: string): string {
    return direction === 'Sent' ? 'arrow_upward' : 'arrow_downward';
  }

  /**
   * Vérifie si la longueur d'un tableau est zéro
   */
  isEmpty(length: number): boolean {
    return length === 0;
  }

  /**
   * Helpers pour l'affichage
   */
  getMethodColor(method: string): string {
    return getMethodColor(method);
  }

  getStatusColor(status: number): string {
    return getStatusColor(status);
  }

  getStatusText(status: number): string {
    const statusTexts: { [key: number]: string } = {
      200: 'OK',
      201: 'Created',
      204: 'No Content',
      301: 'Moved Permanently',
      302: 'Found',
      304: 'Not Modified',
      400: 'Bad Request',
      401: 'Unauthorized',
      403: 'Forbidden',
      404: 'Not Found',
      500: 'Internal Server Error',
      502: 'Bad Gateway',
      503: 'Service Unavailable'
    };
    return statusTexts[status] || 'Unknown';
  }

  /**
   * Formatte un body JSON pour l'affichage
   */
  formatJson(body: string | undefined): string {
    if (!body) return 'No body';

    try {
      const parsed = JSON.parse(body);
      return JSON.stringify(parsed, null, 2);
    } catch (e) {
      // Not JSON, return as-is
      return body;
    }
  }

  /**
   * Vérifie si le body est du JSON
   */
  isJsonBody(body: string | undefined): boolean {
    if (!body) return false;

    try {
      JSON.parse(body);
      return true;
    } catch (e) {
      return false;
    }
  }

  /**
   * Copie du texte dans le presse-papiers
   */
  async copyToClipboard(text: string, label: string): Promise<void> {
    try {
      await navigator.clipboard.writeText(text);
      this.snackBar.open(`${label} copied to clipboard!`, 'Close', {
        duration: 2000
      });
    } catch (err) {
      console.error('Failed to copy:', err);
      this.snackBar.open('Failed to copy to clipboard', 'Close', {
        duration: 2000,
        panelClass: ['error-snackbar']
      });
    }
  }

  /**
   * Copie la requête complète
   */
  copyRequest(): void {
    if (!this.httpInteraction) return;

    const req = this.httpInteraction.request;
    const text = `${req.method} ${req.url}

Headers:
${Object.entries(req.headers).map(([k, v]) => `${k}: ${v}`).join('\n')}

Body:
${req.body || '(empty)'}`;

    this.copyToClipboard(text, 'Request');
  }

  /**
   * Copie la réponse complète
   */
  copyResponse(): void {
    if (!this.httpInteraction) return;

    const res = this.httpInteraction.response;
    const text = `HTTP ${res.status} ${this.getStatusText(res.status)}

Headers:
${Object.entries(res.headers).map(([k, v]) => `${k}: ${v}`).join('\n')}

Body:
${res.body || '(empty)'}`;

    this.copyToClipboard(text, 'Response');
  }

  /**
   * Génère la commande cURL
   */
  getCurlCommand(): string {
    if (!this.httpInteraction) return '';

    const req = this.httpInteraction.request;
    let cmd = `curl -X ${req.method} '${req.url}'`;

    // Add headers
    for (const [key, value] of Object.entries(req.headers)) {
      cmd += ` \\\n  -H '${key}: ${value}'`;
    }

    // Add body
    if (req.body) {
      cmd += ` \\\n  -d '${req.body}'`;
    }

    return cmd;
  }

  /**
   * Copie la commande cURL
   */
  copyCurlCommand(): void {
    const cmd = this.getCurlCommand();
    this.copyToClipboard(cmd, 'cURL command');
  }

  /**
   * Copie tous les messages WebSocket
   */
  copyWebSocketMessages(): void {
    if (!this.wsInteraction) return;

    const text = this.wsInteraction.messages
      .map((msg, idx) => {
        const time = this.formatTimestamp(msg.timestampMs);
        return `[${idx}] ${time} ${msg.direction} (${msg.msgType}): ${msg.data}`;
      })
      .join('\n');

    this.copyToClipboard(text, 'WebSocket messages');
  }

  /**
   * Formate un timestamp en durée relative
   */
  formatTimestamp(timestampMs: number): string {
    const seconds = timestampMs / 1000;

    if (seconds < 1) {
      return `${timestampMs}ms`;
    } else if (seconds < 60) {
      return `${seconds.toFixed(2)}s`;
    } else {
      const minutes = Math.floor(seconds / 60);
      const remainingSeconds = (seconds % 60).toFixed(0);
      return `${minutes}m ${remainingSeconds}s`;
    }
  }

  /**
   * Navigation
   */
  goBack(): void {
    this.router.navigate(['/cassettes', this.cassetteName]);
  }

  /**
   * Actions sur l'interaction
   */
  async deleteInteraction(): Promise<void> {
    if (!confirm('Are you sure you want to delete this interaction?')) {
      return;
    }

    try {
      // TODO: Implement delete via Hydra operation
      this.snackBar.open('Delete not yet implemented', 'Close', { duration: 2000 });
    } catch (err: any) {
      console.error('Failed to delete interaction:', err);
      this.snackBar.open(`Error: ${err.message}`, 'Close', {
        duration: 5000,
        panelClass: ['error-snackbar']
      });
    }
  }

  async editInteraction(): Promise<void> {
    // TODO: Implement edit via Hydra operation
    this.snackBar.open('Edit not yet implemented', 'Close', { duration: 2000 });
  }
}
