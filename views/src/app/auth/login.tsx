import {Button, Divider, PasswordInput, TextInput} from "@mantine/core";
import {Form, useForm} from "@mantine/form";
import {notifications} from "@mantine/notifications";
import useUserContext from "@/store/useUserContext";
import {useNavigate} from "react-router-dom";
import axios from "axios";

export default function LoginPage() {
    const nav = useNavigate();
    const form = useForm({
        mode: "uncontrolled",
        initialValues: {
            username: "",
            password: "",
        },
        validate: {
            username: (value) => {
                if (value.length < 4) {
                    return 'UserName last has 4 len';
                }
                return null;
            },
            password: (value) => {
                if (value.length < 6) {

                    return 'Password last has 6 len';
                }
                return null;
            },
        },
    });

    const user = useUserContext();
    const Submit = async () => {
        const payload = {
            username: form.getValues().username,
            password: form.getValues().password,
        };
        axios
            .post("/api/auth/login", payload)
            .then(res=> {
                if (res.status !== 200) {
                    notifications.show({
                        title: 'Network error',
                        message: res.status,
                        color: 'red',
                        autoClose: 5000,
                    });
                }
                if (res.data.code === 200) {
                    user.setUser(res.data.data);
                    user.setLogin(true);
                    nav("/");
                } else {
                    notifications.show({
                        id: 'login-error',
                        title: 'Login error',
                        message: res.data.message,
                        color: 'red',
                        autoClose: 5000,
                    });
                }
            })
    }
    return (
        <>
            <h1>Sign in</h1>
            <Form form={form} className="form">
                <div>
                    <TextInput label="Account" placeholder="Please input you username or email"
                               key={form.key('username')} {...form.getInputProps('username')}/>
                    <PasswordInput label="Password" placeholder="Please input you password"
                                   key={form.key('password')} {...form.getInputProps("password")}
                    />
                </div>
                <Button type="button" color="orange" onClick={() => {
                    const validate = form.validate();
                    if (validate.hasErrors) {
                        notifications.show({
                            title: 'Login Error',
                            message: "Please check username or password error",
                            color: 'red',
                        });
                        return;
                    }
                    Submit();
                }}>Login</Button>
                <Divider label="Need an account?"/>
                <Button type="button" color="dark" onClick={() => {
                    nav('/auth/register')
                }}>Sign up</Button>
            </Form>
        </>

    );
}