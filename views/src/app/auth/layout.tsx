import {Outlet} from "react-router-dom";


export default function AuthLayout() {
    return (
        <div className="auth">
            <div className="auth-body">
                <Outlet/>
            </div>
        </div>
    );
}