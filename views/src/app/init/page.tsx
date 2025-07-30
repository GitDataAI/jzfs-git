import {HeaderShell} from "@/component/shell/Header.tsx";
import {Form, useForm} from "@mantine/form";
import {Button, Checkbox, TextInput} from "@mantine/core";
import useUserContext from "@/store/useUserContext.tsx";
import axios from "axios";
import {notifications} from "@mantine/notifications";
export interface InitParam {
    name: string,
    description: string,
    initial: boolean,
    is_public: boolean,
}


export const InitPage = () => {
    const from = useForm<InitParam>(
        {
            initialValues: {
                name: "",
                description: "",
                initial: true,
                is_public: true,
            },
            validate: {
                name: (value) => {
                    if (value.length < 4) {
                        return 'Name last has 4 len';
                    }
                    return null;
                },
            },
        }
    );
    const user = useUserContext();
    const Submit = async () => {
        axios.post("/api/repo/init", {
            name: from.getValues().name,
            description: from.getValues().description,
            initial: from.getValues().initial,
            is_public: from.getValues().is_public,
        })
        .then((res) => {
            if (res.data.code === 200) {
                notifications.show({
                    id: 'repo-init',
                    title: 'Init Repo',
                    message: res.data.message,
                    color: 'green',
                })
            }else {
                notifications.show({
                    id: 'repo-init',
                    title: 'Init Repo',
                    message: res.data.message,
                    color: 'red',
                })
            }
        })
        .catch((err) => {
            notifications.show({
                id: 'repo-init',
                title: 'Init Repo',
                message: err.message,
                color: 'red',
            });
        });
    }
    return(
        <>
            <HeaderShell/>
            {
                user.data && (
                    <div style={{
                        position: 'absolute',
                        marginTop: '2rem',
                        fontSize: '2rem',
                        width: '40%',
                        left: '50%',
                        transform: 'translateX(-50%)',
                    }}>
                        <h1 style={{
                            textAlign: 'center',
                            marginBottom: '2rem',
                            fontSize: '2rem',
                        }}>Initial Repository</h1>
                        <Form form={from}>
                            <TextInput style={{ width: '100%' }} label="Owner" placeholder="Nickname" value={user.data.username} disabled />
                            <TextInput style={{ width: '100%' }} label="Repository Name" placeholder="Name" {...from.getInputProps('name')} />
                            <TextInput style={{ width: '100%' }} label="Description" placeholder="Description" {...from.getInputProps('description')} />
                            <Checkbox
                                style={{
                                    marginTop: '1.5rem',
                                    width: '100%'
                                }}
                                defaultChecked
                                label="Public Repository"
                                {...from.getInputProps('public')}
                            />
                           <Checkbox
                                style={{
                                    marginTop: '1.5rem',
                                    width: '100%'
                                }}
                                defaultChecked
                                label="Auto Initial Repository"
                                {...from.getInputProps('initial')}
                            />
                            <Button style={{
                                marginTop: '1.5rem',
                            }} type="button" onClick={Submit}>Create</Button>
                        </Form>
                    </div>
                )
            }
        </>
    )
}