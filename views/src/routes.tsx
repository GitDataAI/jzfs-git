import {createBrowserRouter, RouterProvider} from "react-router-dom";
import AuthLayout from "./app/auth/layout.tsx";
import LoginPage from "@/app/auth/login.tsx";
import RegisterPage from "@/app/auth/register.tsx";
import {RootLayout} from "@/app/layout.tsx";
import {RootRepoList} from "@/app/root/RepoList.tsx";
import {UsersList} from "@/app/root/UsersList.tsx";
import {InitPage} from "@/app/init/page.tsx";
import {Repolayout} from "@/app/repository/layout.tsx";
import {RepoFiles} from "@/app/repository/files.tsx";
import {RepoBranch} from "@/app/repository/branch.tsx";
import {RepoCommit} from "@/app/repository/commit.tsx";

export default function AppRoutes() {
    const router = createBrowserRouter([
        {
            path: '/auth',
            element: <AuthLayout/>,
            children: [
                {
                    path: 'login',
                    element: <LoginPage/>
                },
                {
                    path: 'register',
                    element: <RegisterPage/>
                }
            ]
        },
        {
            path: '/',
            element: <RootLayout/>,
            children: [
                {
                    path: '',
                    element: <RootRepoList/>
                },
                {
                    path: 'users',
                    element: <UsersList/>
                }
            ]
        },
        {
            path: '/init',
            element: <InitPage/>
        },
        {
            path: '/:owner/:repo',
            element: <Repolayout/>,
            children: [
                {
                    path: '',
                    element: <RepoFiles/>
                },
                {
                    path: 'branches',
                    element: <RepoBranch/>
                },
                {
                    path: 'commits',
                    element: <RepoCommit/>
                },
                {
                    path: 'tree/*',
                    element: <RepoFiles/>
                }
            ]
        },
        {
            path: '*',
            element: <div>404</div>
        }
    ])
    return(
        <RouterProvider router={router}/>
    )
}