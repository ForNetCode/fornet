import {Layout} from 'antd'
import {Navigate, Outlet, useLocation} from "react-router-dom"
import './AppLayout.less'
import AppBreadcrumb from "./AppBreadcrumb"
import {RequireAuth} from "../auth";


const {Header, Footer, Content} = Layout
export default function AppLayout() {
    const location = useLocation()
    if (location.pathname === '/') {
        return <Navigate to={'/network'} replace={true}/>
    }
    return (
        <Layout style={{height: 'calc(100vh)'}}>
            <Header style={{textAlign: 'center'}}>
                <span style={{fontWeight: 'bold', fontSize: '20px'}}>ForNet Manager (BETA)</span>
            </Header>
            <Content style={{padding: '0 50px'}}>
                <AppBreadcrumb/>
                <div className="site-layout-content">
                    <RequireAuth>
                        <Outlet/>
                    </RequireAuth>
                </div>
            </Content>
            <Footer style={{textAlign: 'center', background: '#16191B'}}>ForNet ©2023 Created by Timzaak</Footer>
        </Layout>)

}
