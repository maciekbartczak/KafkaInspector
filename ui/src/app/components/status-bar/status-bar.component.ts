import { Component, Input } from "@angular/core";
import { AppStateService } from "../../app-state.service";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";

@Component({
  selector: "app-status-bar",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./status-bar.component.html",
  styleUrl: "./status-bar.component.css",
})
export class StatusBarComponent {
  @Input()
  isConnected: boolean | null = false;

  constructor(private state: AppStateService) {}

  disconnect(): void {
    invoke<boolean>("disconnect")
      .then(() => {
        this.state.setIsConnected(false);
      })
      .catch((err) => {
        console.error(`error while disconnecting from cluster: ${err}`);
      });
  }
}
