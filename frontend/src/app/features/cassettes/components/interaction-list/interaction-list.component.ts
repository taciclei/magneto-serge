import { Component, Input, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Observable, of } from 'rxjs';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatCardModule } from '@angular/material/card';
import { MatChipsModule } from '@angular/material/chips';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTabsModule } from '@angular/material/tabs';

import { InteractionResource } from '../../../../core/models/interaction.model';
import { AlcaeusService } from '../../../../core/services/alcaeus.service';

/**
 * Component affichant la liste des interactions d'une cassette
 *
 * Utilise expansion panels pour afficher les détails de chaque interaction
 */
@Component({
  selector: 'app-interaction-list',
  standalone: true,
  imports: [
    CommonModule,
    MatExpansionModule,
    MatCardModule,
    MatChipsModule,
    MatIconModule,
    MatButtonModule,
    MatProgressSpinnerModule,
    MatTabsModule
  ],
  templateUrl: './interaction-list.component.html',
  styleUrl: './interaction-list.component.scss'
})
export class InteractionListComponent implements OnInit {
  @Input() cassetteName!: string;

  interactions$: Observable<InteractionResource[]> = of([]);
  loading = false;
  error: string | null = null;

  constructor(private alcaeusService: AlcaeusService) {}

  ngOnInit(): void {
    if (this.cassetteName) {
      this.loadInteractions();
    }
  }

  /**
   * Charge les interactions depuis l'API Hydra
   */
  loadInteractions(): void {
    this.loading = true;
    this.error = null;

    const url = `/cassettes/${this.cassetteName}/interactions`;

    this.alcaeusService.loadResource(url).subscribe({
      next: (response) => {
        const collection = response.root as any;
        const interactions = collection['hydra:member'] || [];
        this.interactions$ = of(interactions);
        this.loading = false;
      },
      error: (err) => {
        this.error = err.message || 'Erreur lors du chargement des interactions';
        this.loading = false;
        this.interactions$ = of([]);
      }
    });
  }

  /**
   * Retourne la classe CSS pour le type d'interaction
   */
  getInteractionTypeClass(type: string): string {
    return type === 'Http' ? 'http-type' : 'websocket-type';
  }

  /**
   * Retourne la classe CSS pour le statut HTTP
   */
  getStatusClass(status: number): string {
    if (status >= 200 && status < 300) {
      return 'status-success';
    } else if (status >= 300 && status < 400) {
      return 'status-redirect';
    } else if (status >= 400 && status < 500) {
      return 'status-client-error';
    } else {
      return 'status-server-error';
    }
  }

  /**
   * Formate les headers pour affichage
   */
  formatHeaders(headers: Record<string, string>): string {
    return Object.entries(headers)
      .map(([key, value]) => `${key}: ${value}`)
      .join('\n');
  }

  /**
   * Formate le body pour affichage
   */
  formatBody(body: number[] | null): string {
    if (!body || body.length === 0) {
      return '(vide)';
    }

    // Convertir le tableau d'octets en string
    try {
      const decoder = new TextDecoder('utf-8');
      const uint8Array = new Uint8Array(body);
      const text = decoder.decode(uint8Array);

      // Tenter de parser en JSON pour un affichage formaté
      try {
        const json = JSON.parse(text);
        return JSON.stringify(json, null, 2);
      } catch {
        return text;
      }
    } catch {
      return `Binary data (${body.length} bytes)`;
    }
  }

  /**
   * Formate la direction du message WebSocket
   */
  formatDirection(direction: string): string {
    return direction === 'Sent' ? 'Envoyé' : 'Reçu';
  }

  /**
   * Retourne l'icône pour la direction
   */
  getDirectionIcon(direction: string): string {
    return direction === 'Sent' ? 'arrow_upward' : 'arrow_downward';
  }

  /**
   * Formate le timestamp en durée relative
   */
  formatTimestamp(timestampMs: number): string {
    const seconds = timestampMs / 1000;
    if (seconds < 1) {
      return `${timestampMs}ms`;
    } else {
      return `${seconds.toFixed(2)}s`;
    }
  }
}
