import { Component } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/core";
import { AppStateService } from "../../app-state.service";

@Component({
  selector: "app-connect-to-cluster",
  standalone: true,
  imports: [FormsModule],
  templateUrl: "./connect-to-cluster.component.html",
  styleUrl: "./connect-to-cluster.component.css",
})
export class ConnectToClusterComponent {
  clusterAddress: string = "localhost:9092";

  constructor(private state: AppStateService) {}

  connect(): void {
    console.log("connecting to cluster");
    invoke<boolean>("connect", { params: { address: this.clusterAddress } })
      .then(() => {
        this.state.setIsConnected(true);
      })
      .catch((err) => {
        console.error(`error while connecting to cluster: ${err}`);
      });
  }
}
