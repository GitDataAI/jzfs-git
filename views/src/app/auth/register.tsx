import {Button, Divider, PasswordInput, TextInput} from "@mantine/core";
import {Form, useForm} from "@mantine/form";
import {notifications} from "@mantine/notifications";
import useUserContext from "@/store/useUserContext";
import {useNavigate} from "react-router-dom";
import axios from "axios";

export default function RegisterPage() {
    const nav = useNavigate();
    const form = useForm({
        mode: "uncontrolled",
        initialValues: {
            username: "",
            password: "",
            email: "",
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
            email: (value) => {
                if (!value.includes("@")) {
                    return 'Email is invalid';
                }
                return null
            },
        },
    });

    const _user = useUserContext();
    const Submit = async () => {
        const payload =  {
            username: form.getValues().username,
            password: form.getValues().password,
            email: form.getValues().email,
        };
        axios.post("/api/auth/register", payload)
            .then(res=> {
                if (res.data.code === 200) {
                    _user.setUser(res.data.data);
                    _user.setLogin(true);
                    nav("/");
                } else {
                    notifications.show({
                        id: 'register-error',
                        title: 'Register error',
                        message: res.data.message,
                        color: 'red',
                        autoClose: 5000,
                    });
                }
            })
            .catch(err=> {
                console.log(err);
            });
    }
    return (
        <>
            <h1>Sign in</h1>
            <Form form={form} className="form">
                <div>
                    <TextInput label="Account" placeholder="Please input you username or email"
                               key={form.key('username')} {...form.getInputProps('username')}/>
                    <TextInput label="Email" placeholder="Please input you email"
                               key={form.key('email')} {...form.getInputProps('email')}/>
                    <PasswordInput label="Password" placeholder="Please input you password" key={form.key('password')} {...form.getInputProps("password")}/>
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
                }}>Register</Button>
                <Divider label="Need an account?"/>
                <Button type="button" color="dark" onClick={() => {
                    nav('/auth/login')
                }}>Login</Button>
            </Form>
        </>

    );
}