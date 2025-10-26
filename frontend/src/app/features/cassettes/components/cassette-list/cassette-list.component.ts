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
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatTooltipModule } from '@angular/material/tooltip';
import { Router } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { debounceTime, distinctUntilChanged, Subject } from 'rxjs';

import { CassetteActions } from '../../state/cassette.actions';
import {
  selectCassettes,
  selectLoading,
  selectError,
  selectPaginationInfo,
  selectSearch,
  selectSortBy,
  selectSortOrder
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
    FormsModule,
    MatTableModule,
    MatProgressSpinnerModule,
    MatButtonModule,
    MatIconModule,
    MatCardModule,
    MatPaginatorModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
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
  search$: Observable<string | null>;
  sortBy$: Observable<string | null>;
  sortOrder$: Observable<string>;

  // Form values
  searchQuery: string = '';
  selectedSortBy: string = '';
  selectedSortOrder: string = 'asc';

  // Subject pour debounce de recherche
  private searchSubject = new Subject<string>();

  // Options de tri
  sortOptions = [
    { value: '', label: 'Défaut' },
    { value: 'name', label: 'Nom' },
    { value: 'date', label: 'Date' },
    { value: 'interactions', label: 'Interactions' }
  ];

  sortOrderOptions = [
    { value: 'asc', label: 'Croissant' },
    { value: 'desc', label: 'Décroissant' }
  ];

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
    private router: Router
  ) {
    this.cassettes$ = this.store.select(selectCassettes);
    this.loading$ = this.store.select(selectLoading);
    this.error$ = this.store.select(selectError);
    this.paginationInfo$ = this.store.select(selectPaginationInfo);
    this.search$ = this.store.select(selectSearch);
    this.sortBy$ = this.store.select(selectSortBy);
    this.sortOrder$ = this.store.select(selectSortOrder);
  }

  ngOnInit(): void {
    // Synchroniser les form values avec le store
    this.search$.subscribe(search => this.searchQuery = search || '');
    this.sortBy$.subscribe(sortBy => this.selectedSortBy = sortBy || '');
    this.sortOrder$.subscribe(sortOrder => this.selectedSortOrder = sortOrder);

    // Debounce pour la recherche (300ms)
    this.searchSubject.pipe(
      debounceTime(300),
      distinctUntilChanged()
    ).subscribe(search => {
      this.store.dispatch(CassetteActions.updateSearch({
        search: search || null
      }));
    });

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

  /**
   * Déclenché quand l'utilisateur tape dans la recherche
   */
  onSearchChange(value: string): void {
    this.searchSubject.next(value);
  }

  /**
   * Déclenché quand l'utilisateur change le critère de tri
   */
  onSortByChange(value: string): void {
    this.store.dispatch(CassetteActions.updateSort({
      sortBy: value || null,
      sortOrder: this.selectedSortOrder
    }));
  }

  /**
   * Déclenché quand l'utilisateur change l'ordre de tri
   */
  onSortOrderChange(value: string): void {
    this.store.dispatch(CassetteActions.updateSort({
      sortBy: this.selectedSortBy || null,
      sortOrder: value
    }));
  }

  /**
   * Efface tous les filtres et la recherche
   */
  clearFilters(): void {
    this.searchQuery = '';
    this.selectedSortBy = '';
    this.selectedSortOrder = 'asc';
    this.store.dispatch(CassetteActions.clearFilters());
  }
}
