import { Component } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: "app-connect-to-cluster",
  standalone: true,
  imports: [FormsModule],
  templateUrl: "./connect-to-cluster.component.html",
  styleUrl: "./connect-to-cluster.component.css",
})
export class ConnectToClusterComponent {
  clusterAddress: string = "localhost:9092";

  connect(): void {
    invoke<boolean>("connect", { params: { address: this.clusterAddress } })
      .then(() => {
        console.log("connected successfully");
      })
      .catch((err) => {
        console.error(`error while connecting to cluster: ${err}`);
      });
  }
}
