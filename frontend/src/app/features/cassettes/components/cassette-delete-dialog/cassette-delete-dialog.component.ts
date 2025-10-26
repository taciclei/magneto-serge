import { Component, inject, Inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatDialogModule, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { Store } from '@ngrx/store';
import { CassetteActions } from '../../state/cassette.actions';

/**
 * Data passed to the delete dialog
 */
export interface DeleteDialogData {
  cassetteName: string;
  interactionCount?: number;
}

/**
 * Dialog component for deleting a cassette
 *
 * Shows confirmation dialog with cassette details before deletion
 */
@Component({
  selector: 'app-cassette-delete-dialog',
  standalone: true,
  imports: [
    CommonModule,
    MatDialogModule,
    MatButtonModule,
    MatIconModule
  ],
  templateUrl: './cassette-delete-dialog.component.html',
  styleUrl: './cassette-delete-dialog.component.scss'
})
export class CassetteDeleteDialogComponent {
  private store = inject(Store);
  private dialogRef = inject(MatDialogRef<CassetteDeleteDialogComponent>);

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DeleteDialogData
  ) {}

  /**
   * Confirm deletion and dispatch delete action
   */
  onConfirm(): void {
    this.store.dispatch(CassetteActions.deleteCassette({
      name: this.data.cassetteName
    }));

    this.dialogRef.close(true);
  }

  /**
   * Cancel deletion and close dialog
   */
  onCancel(): void {
    this.dialogRef.close(false);
  }
}
