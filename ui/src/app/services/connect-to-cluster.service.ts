import { Injectable } from "@angular/core";
import { Observable, from } from "rxjs";
import { invoke } from "@tauri-apps/api/core";

@Injectable({
  providedIn: "root",
})
export class ConnectToClusterService {
  public connectToCluster(params: { address: string }): Observable<boolean> {
    return from(
      invoke<boolean>("connect", { params: { address: params.address } }),
    );
  }
}
