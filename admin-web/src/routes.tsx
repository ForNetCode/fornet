import {createBrowserRouter, RouterProvider,} from "react-router-dom";
import NetworkListPage from "./view/network/NetworkListPage";
import AppLayout from "./layout/AppLayout";
import NotFoundPage from "./view/NotFoundPage";
import {CreateNetworkPage} from "./view/network/CreateNetworkPage";
import NetworkDetailPage from "./view/network/NetworkDetailPage";
import NodeListPage from "./view/node/NodeListPage";
import {CreateNodePage} from "./view/node/CreateNodePage";
import NodeDetailPage from "./view/node/NodeDetailPage";
import intl from "./local/intl";
import LoginPage from "./view/login/LoginPage";
import {AuthProvider} from "./auth";

//https://github.com/keycloak/keycloak/issues/8959
// could not use `createHashRouter`

const appRouter = createBrowserRouter([
    {
        path: '/',
        element: <AppLayout/>,
        children: [
            {
                path: 'network',
                handle: {
                    crumb: () => intl.formatMessage({id: 'nav.network'}),
                },
                children: [{
                    index: true,
                    element: <NetworkListPage/>,
                }, {
                    path: 'create',
                    element: <CreateNetworkPage/>,
                    handle: {
                        crumb: () => intl.formatMessage({id: 'create'}),
                    }
                }, {
                    path: ':networkId',
                    handle: {
                        crumb: (data: any) => data.params.networkId,
                    },
                    children: [{
                        index: true,
                        element: <NetworkDetailPage/>,
                    },
                        {
                            path: 'node',
                            handle: {
                                crumb: () => intl.formatMessage({id: 'nav.node'}),
                            },
                            children: [
                                {
                                    index: true,
                                    element: <NodeListPage/>,
                                },
                                {
                                    path: 'create',
                                    element: <CreateNodePage/>,
                                    handle: {
                                        crumb: () => intl.formatMessage({id: 'create'}),
                                    }
                                }, {
                                    path: ':nodeId',
                                    element: <NodeDetailPage/>,
                                    handle: {
                                        crumb: (data: any) => data.params.nodeId,
                                    }
                                }
                            ]
                        },]
                },]
            }
        ],
    },
    {
        path: '/login',
        element: <LoginPage/>,
    },
    {
        path: '/*',
        element: <NotFoundPage/>
    },

])

export default function AppRouter() {
    return (
        <AuthProvider>
            <RouterProvider router={appRouter}/>
        </AuthProvider>)
}
