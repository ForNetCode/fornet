import React from "react";
import {Navigate, useLocation} from "react-router-dom";
import {clearPersistenceToken, getPersistenceToken} from "./api/authAPI";


export interface AuthContextType {
    isLogin: boolean
    signIn: () => void
    singOut: () => void
}

let AuthContext = React.createContext<AuthContextType>(null!);

export function AuthProvider({children}: { children: React.ReactNode }) {
    const [authInfo, setAuthInfo] = React.useState<AuthContextType>(() => {
        return {
            isLogin: !!getPersistenceToken(),
            signIn: () => {
                setAuthInfo({
                    isLogin: true,
                    signIn: authInfo.signIn,
                    singOut: authInfo.singOut
                })
            },
            singOut: () => {
                clearPersistenceToken()
                window.location.reload()
            }
        }
    });

    return <AuthContext.Provider value={authInfo}>
        {children}
    </AuthContext.Provider>
}

export function useAuth() {
    return React.useContext(AuthContext);
}

export function RequireAuth({children}: { children: JSX.Element }) {
    let auth = useAuth();
    let location = useLocation();
    if (!auth.isLogin) {
        return <Navigate to="/login" state={{from: location}} replace={true}/>
    }
    return children
}
