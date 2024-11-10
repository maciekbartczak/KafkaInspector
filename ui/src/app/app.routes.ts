import {Routes} from "@angular/router";
import {ConnectToClusterComponent} from "./components/connect-to-cluster/connect-to-cluster.component";
import {TopicListComponent} from "./components/topic-list/topic-list.component";
import {reconnectGuard} from "./reconnect.guard";

export const routes: Routes = [
    {
        path: '',
        canActivate: [reconnectGuard],
        children: [
            {
                path: 'connect',
                component: ConnectToClusterComponent
            },
            {
                path: 'topics',
                component: TopicListComponent
            }
        ]
    },
];
