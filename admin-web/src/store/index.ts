import {Action, configureStore, ThunkAction} from "@reduxjs/toolkit";
//import {composeWithDevTools} from "@reduxjs/toolkit/dist/devtoolsExtension";
import {TypedUseSelectorHook, useDispatch, useSelector} from "react-redux";

export const store = configureStore({
    reducer: {},
    //enhancers: composeWithDevTools({})
})

export type AppDispatch = typeof store.dispatch
export type RootState = ReturnType<typeof store.getState>
export type AppThunk<ReturnType = void> = ThunkAction<ReturnType,
    RootState,
    unknown,
    Action<String>>

export const useAppDispatch = () => useDispatch<AppDispatch>()
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector
