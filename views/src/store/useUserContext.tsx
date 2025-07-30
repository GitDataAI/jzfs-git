import {create} from "zustand/react";
import {createJSONStorage, devtools, persist} from "zustand/middleware";
import axios from "axios";


export interface UserState {
    data?: {
        uid: string,
        username: string,
        email: string,
    },
    isLogin: boolean,
    setUser: (data: {
        uid: string,
        username: string,
        email: string,
    }) => void,
    setLogin: (isLogin: boolean) => void,
    logout: () => void,
    getUser: () => {
        uid: string,
        username: string,
        email: string,
    } | undefined,
    getLogin: () => boolean,
    refresh: () => void,
}

const useUserContext = create<UserState>()(
    devtools(
        persist(
            (set, get) => (
                {
                    data: undefined,
                    isLogin: false,
                    setUser: (data) => {
                        set({
                            data: data,
                            isLogin: true,
                        })
                    },
                    setLogin: (isLogin) => {
                        set({
                            isLogin: isLogin,
                        })
                    },
                    logout: () => {
                        fetch('/api/auth/logout', {
                            method: 'POST',
                        }).then().catch().finally();
                        set({
                            data: undefined,
                            isLogin: false,
                        })
                    },
                    getUser: () => {
                        return get().data;
                    },
                    getLogin: () => {
                        return get().isLogin;
                    },
                    refresh() {
                        axios.post("/api/auth/context")
                        .then(res=> {
                            set({
                                data: res.data.data,
                                isLogin: res.data.code === 200
                            });
                        })
                    }
                }
            ),
            {
                name: "user",
                storage: createJSONStorage(() => localStorage)
            }
        )
    )
);


export default useUserContext;