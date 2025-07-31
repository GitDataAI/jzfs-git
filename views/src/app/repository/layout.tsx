import { useEffect, useState } from "react";
import {Outlet, useNavigate, useParams,} from "react-router-dom";
import { HeaderShell } from "@/component/shell/Header.tsx";
import axios from "axios";
import {Tabs} from "@mantine/core";
import { Box, Card, Text, Button, Group } from "@mantine/core";
import {AlertCircle, CopyIcon, Loader2} from "lucide-react";
import {useClipboard} from "@mantine/hooks";
import {notifications} from "@mantine/notifications";

interface RepoApiResponse {
    code: number;
    data: RepoData;
    message: string;
}

interface RepoData {
    branches: Branch[];
    repo: Repository;
}

export interface Repository {
    created_at: string;
    deleted_at: string | null;
    description: string;
    name: string;
    owner: string;
    uid: string;
    updated_at: string;
}

interface Branch {
    default: boolean;
    head: string;
    name: string;
    time: string;
}


export const Repolayout = () => {
    const { owner, repo } = useParams<{ owner: string; repo: string }>();
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [repoData, setRepoData] = useState<RepoData | null>(null);
    const nav = useNavigate();
    const [type, setType] = useState('files');
    const clip = useClipboard()
    useEffect(() => {
        const urls = window.location.href.split("/");
        setType("files")
        for (let i = urls.length - 1; i >= 0; i--) {
            if (urls[i] === "files") {
                setType("files");
                break;
            } else if (urls[i] === "branches") {
                setType("branches");
                break;
            } else if (urls[i] === "commits") {
                setType("commits");
                break;
            } else if (urls[i] === "settings") {
                setType("settings");
            }
        }
    }, [window.location.href]);
    const FetchDashData = () => {
        if (!owner || !repo) return;
        
        setLoading(true);
        let url = `/api/repo/${owner}/${repo}`;
        axios.get(url)
            .then(res => {
                const data: RepoApiResponse = res.data;
                if (data.code === 200 && data.data) {
                    setRepoData(data.data);
                    setError(null);
                } else {
                    setError(data.message || 'Failed to load repository data');
                }
            })
            .catch(err => {
                setError(err.message || 'Network error occurred');
            })
            .finally(() => {
                setLoading(false);
            });
    };

    useEffect(() => {
        FetchDashData();
    }, [owner, repo]);

    if (loading) {
        return (
            <>
                <HeaderShell />
                <Box p={4} mt={64}>
                    <Card>
                        <Box p={6} display="flex">
                            <Loader2 size={48} className="mr-2" color="blue" />
                            <Text mt={4} size="lg">Loading repository data...</Text>
                        </Box>
                    </Card>
                </Box>
            </>
        );
    }

    if (error || !repoData) {
        return (
            <>
                <HeaderShell />
                <Box p={4} mt={64}>
                    <Card>
                        <Box p={6} display="flex">
                            <AlertCircle size={48} className="mr-2" color="red" />
                            <Text mt={4} size="lg" >{error || 'Unknown error occurred'}</Text>
                            <Button mt={4} onClick={FetchDashData}>Try Again</Button>
                        </Box>
                    </Card>
                </Box>
            </>
        );
    }

    const { repo: repository } = repoData;

    function copyText(arg0: string) {
        clip.copy(arg0)
        notifications.show({
            message: "Copy Success",
            color: "green"
        })
    }

    return (
        <>
            <HeaderShell />
            <Box mt={64} p={4} style={{ maxWidth: '1200px', margin: '0 auto', width: '100%' }}>
                <Card mb={4} >
                    <Box p={4}>
                        <Group justify="space-between" mb={2}>
                            <Text size="2xl">{owner}/{repository.name}</Text>
                            <Text color="dimmed">
                                {new Date(repository.updated_at).toLocaleDateString()}
                            </Text>
                        </Group>
                        <Box style={{
                            justifyContent: "space-between",
                            display: "flex",
                            alignItems: "center",
                        }}>
                            <Text color="dimmed" mb={4}>{repository.description || 'No description provided'}</Text>
                            <div style={{
                                display: "flex",
                                alignItems: "center",
                                gap: "1rem",
                            }}>
                                <div style={{
                                    border: "1px solid #ccc",
                                    borderRadius: "4px",
                                    padding: "0.5rem",
                                    display: "flex",
                                    backgroundColor: "#f5f5f5",
                                    alignItems: "center",
                                    justifyContent: "space-between",
                                    marginBottom: "0.5rem",
                                    gap: "0.5rem"
                                }}>
                                    {window.location.protocol + "//"}{window.location.host + "/git/" + owner + "/" + repository.name + ".git"}
                                    <CopyIcon onClick={() => {
                                        copyText(window.location.protocol + "//" + window.location.host + "/git/" + owner + "/" + repository.name + ".git");
                                    }}/>
                                </div>
                            </div>
                        </Box>
                    </Box>
                </Card>
                <Tabs defaultValue={type} value={type} onChange={(value) => {
                        if (value === 'files') {
                            nav(`/${owner}/${repository.name}`)
                        } else if (value === 'branches') {
                            nav( `/${owner}/${repository.name}/branches`)
                        } else if (value === 'commits') {
                            nav(`/${owner}/${repository.name}/commits`)
                        } else if (value === 'settings') {
                        }
                    }}>
                        <Tabs.List>
                            <Tabs.Tab value="files">
                                Files
                            </Tabs.Tab>
                            <Tabs.Tab value="branches" >
                                Branches
                            </Tabs.Tab>
                            <Tabs.Tab value="commits" >
                                Commits
                            </Tabs.Tab>
                            <Tabs.Tab value="settings" >
                                Settings
                            </Tabs.Tab>
                        </Tabs.List>
                    <Outlet/>
                    </Tabs>
            </Box>
        </>
    );
}