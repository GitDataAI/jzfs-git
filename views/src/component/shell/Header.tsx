import {Link, useNavigate} from "react-router-dom";
import useUserContext from "@/store/useUserContext.tsx";
import {useEffect, useState} from "react";
import {Button} from "@mantine/core";

export const HeaderShell = () => {
    const user = useUserContext();
    const nav = useNavigate();
    useEffect(() => {
        user.refresh();
    }, []);
    const [Width, setWidth] = useState(window.innerWidth);
    useEffect(() => {
        setWidth(window.innerWidth)
    }, [window.innerWidth]);
    return(
        <div style={{
            background: "white",
            height: "64px",
            width: "100%",
            display: "flex",
            borderBottom: "0.1rem solid rgb(217, 217, 217)",
        }}>
            {
                Width > 768 && (
                    <div style={{
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        padding: "0 1rem",
                    }}>
                        <img src="/jzflow-logo-rust.png" alt="logo" style={{
                            height: "32px",
                        }}/>
                    </div>
                )
            }
            <div style={{
                display: "flex",
                alignItems: "center",
                flex: 1,
                padding: "0 1rem",
                gap: "1rem",
            }}>
                <Link to="/">Repository</Link>
                <Link to="/user">Users</Link>
            </div>

            <div style={{
                display: "flex",
                alignItems: "center",
                flex: 1,
                padding: "0 1rem",
                gap: "1rem",
                justifyContent: "flex-end",
            }}>
                {
                    user.isLogin ? (
                        <>
                            <Button color="gray" onClick={()=> nav("/init")}>New Repository</Button>
                            <Button color="red" onClick={()=>user.logout()}>Logout</Button>
                        </>
                    ) : (
                        <>
                            <Button color="green" onClick={()=>nav("/auth/login")}>Login</Button>
                            <Button color="green" onClick={()=>nav("/auth/register")}>Register</Button>
                        </>
                    )
                }
            </div>
        </div>
    )
}