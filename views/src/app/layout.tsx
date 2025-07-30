import {HeaderShell} from "@/component/shell/Header.tsx";
import {Outlet} from "react-router-dom";

export const RootLayout = () => {
    return(
        <div style={{
            position: "fixed",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
        }}>
            <HeaderShell/>
            <Outlet/>
        </div>
    )
}