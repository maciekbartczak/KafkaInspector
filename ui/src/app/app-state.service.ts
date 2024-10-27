import {Injectable} from "@angular/core";
import {BehaviorSubject, Observable} from "rxjs";
import {Metadata} from "./types";

@Injectable({
  providedIn: "root",
})
export class AppStateService {
  private isConnectedSubject = new BehaviorSubject<boolean>(false);
  private metadataSubject = new BehaviorSubject<Metadata>({topics: []});

  public isConnected$: Observable<boolean> = this.isConnectedSubject.asObservable();
  public metadata$: Observable<Metadata> = this.metadataSubject.asObservable();

  setIsConnected(isConnected: boolean): void {
    this.isConnectedSubject.next(isConnected);
  }

  setMetadata(metadata: Metadata): void {
    this.metadataSubject.next(metadata);
  }

}