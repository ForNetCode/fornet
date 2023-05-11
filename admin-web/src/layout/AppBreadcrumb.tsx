import {Breadcrumb, Row} from 'antd';
import {Link, useMatches} from "react-router-dom";
// const breadcrumbNameMap: Record<string, string> = {
//     '/network': 'nav.network',
//     '/network/create': 'create',
//     '/network/id': 'detail',
// }
export default function AppBreadcrumb() {
    const matches = useMatches();
    // @ts-ignore
    let breadcrumbs = matches.filter((match) => Boolean(match.handle?.crumb))
        .map((match, index) =>{
            return <Breadcrumb.Item key={`k${index}`}>
                <Link to={match.pathname}>{(match.handle!! as any).crumb(match)}</Link>
            </Breadcrumb.Item>
        })
    const breadcrumbItems = [
        <Breadcrumb.Item key="home">
            <Link to="/">Home</Link>
        </Breadcrumb.Item>,
    ].concat(breadcrumbs);
    return(<Row align="middle">
        <span style={{fontWeight:'bold', fontSize:'18px'}}>Navigator</span>&nbsp; :
            <Breadcrumb
            style={{marginLeft: '8px', marginTop: '10px', marginBottom: '10px'}}>
            {breadcrumbItems}
            </Breadcrumb>
    </Row>)

    // return <Breadcrumb
    //     style={{marginLeft: '8px', marginTop: '10px', marginBottom: '10px'}}></Breadcrumb>
    // const location = useLocation();
    // const intl = useIntl()

    //const pathSnippets = location.pathname.split('/').filter(i => i);

    // const extraBreadcrumbItems = pathSnippets.map((_, index) => {
    //     const url = `/${pathSnippets.slice(0, index + 1).join('/')}`
    //     const message = intl.formatMessage({id: breadcrumbNameMap[url]})
    //     return (
    //         <Breadcrumb.Item key={url}>
    //             <Link to={url}>{message}</Link>
    //         </Breadcrumb.Item>
    //     );
    // });
    // const breadcrumbItems = [
    //     <Breadcrumb.Item key="home">
    //         <Link to="/">Home</Link>
    //     </Breadcrumb.Item>,
    // ].concat(extraBreadcrumbItems);
    // return <Breadcrumb
    //     style={{marginLeft: '8px', marginTop: '10px', marginBottom: '10px'}}>{extraBreadcrumbItems}</Breadcrumb>
}
