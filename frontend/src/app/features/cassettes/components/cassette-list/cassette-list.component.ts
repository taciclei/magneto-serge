import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Store } from '@ngrx/store';
import { Observable } from 'rxjs';
import { MatTableModule } from '@angular/material/table';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatCardModule } from '@angular/material/card';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatSnackBar, MatSnackBarModule } from '@angular/material/snack-bar';
import { MatTooltipModule } from '@angular/material/tooltip';
import { Router } from '@angular/router';

import { CassetteActions } from '../../state/cassette.actions';
import {
  selectCassettes,
  selectLoading,
  selectError,
  selectPaginationInfo
} from '../../state/cassette.selectors';
import { CassetteResource } from '../../../../core/models/cassette.model';

/**
 * Component affichant la liste paginée des cassettes
 *
 * Utilise Material Table avec pagination Hydra
 */
@Component({
  selector: 'app-cassette-list',
  standalone: true,
  imports: [
    CommonModule,
    MatTableModule,
    MatProgressSpinnerModule,
    MatButtonModule,
    MatIconModule,
    MatCardModule,
    MatPaginatorModule,
    MatSnackBarModule,
    MatTooltipModule
  ],
  templateUrl: './cassette-list.component.html',
  styleUrl: './cassette-list.component.scss'
})
export class CassetteListComponent implements OnInit {
  // Observables depuis le store
  cassettes$: Observable<CassetteResource[]>;
  loading$: Observable<boolean>;
  error$: Observable<string | null>;
  paginationInfo$: Observable<any>;

  // Configuration de la table
  displayedColumns: string[] = [
    'name',
    'version',
    'recordedAt',
    'interactionCount',
    'sizeBytes',
    'actions'
  ];

  constructor(
    private store: Store,
    private router: Router,
    private snackBar: MatSnackBar
  ) {
    this.cassettes$ = this.store.select(selectCassettes);
    this.loading$ = this.store.select(selectLoading);
    this.error$ = this.store.select(selectError);
    this.paginationInfo$ = this.store.select(selectPaginationInfo);
  }

  ngOnInit(): void {
    // Charger les cassettes au démarrage
    this.store.dispatch(CassetteActions.loadCassettes({ params: { page: 1, limit: 20 } }));
  }

  /**
   * Navigue vers le détail d'une cassette
   */
  viewDetails(cassette: CassetteResource): void {
    this.router.navigate(['/cassettes', cassette.name]);
  }

  /**
   * Recharge les cassettes
   */
  reload(): void {
    this.store.dispatch(CassetteActions.loadCassettes({}));
    this.snackBar.open('Rechargement des cassettes...', '', {
      duration: 1500
    });
  }

  /**
   * Navigation pagination - Page suivante
   */
  nextPage(): void {
    this.store.dispatch(CassetteActions.navigateToNextPage());
  }

  /**
   * Navigation pagination - Page précédente
   */
  previousPage(): void {
    this.store.dispatch(CassetteActions.navigateToPreviousPage());
  }

  /**
   * Navigation pagination - Première page
   */
  firstPage(): void {
    this.store.dispatch(CassetteActions.navigateToFirstPage());
  }

  /**
   * Navigation pagination - Dernière page
   */
  lastPage(): void {
    this.store.dispatch(CassetteActions.navigateToLastPage());
  }

  /**
   * Formate la taille en octets en KB/MB
   */
  formatSize(bytes: number): string {
    if (bytes < 1024) {
      return `${bytes} B`;
    } else if (bytes < 1024 * 1024) {
      return `${(bytes / 1024).toFixed(2)} KB`;
    } else {
      return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
    }
  }

  /**
   * Formate la date ISO en format lisible
   */
  formatDate(isoDate: string): string {
    const date = new Date(isoDate);
    return date.toLocaleString('fr-FR', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
}
