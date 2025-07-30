import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import axios from "axios";
import {AlertCircle, Loader2} from "lucide-react";
import { Button, Group, Text } from "@mantine/core";
import type {Repository} from "@/app/repository/layout.tsx";

interface CommitDataCommits {
    author: string;
    committer: string;
    index: number;
    message: string;
    hash: string;
    timestamp: number;
}

interface CommitData {
    commits: CommitDataCommits[],
    limit: number,
    total: number,
    page: number,
    repo: Repository
}

interface CommitApiResponse {
    code: number;
    data: CommitData;
    message: string;
}

const limit = 10;
export const RepoCommit = () => {
    const { owner, repo } = useParams<{ owner: string; repo: string }>();
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [commits, setCommits] = useState<CommitDataCommits[]>([]);
    const [page, setPage] = useState(1);
    const [total, setTotal] = useState(0);

    const fetchCommits = () => {
        if (!owner || !repo) return;

        setLoading(true);
        const url = `/api/repo/${owner}/${repo}/commits?page=${page}&limit=${limit}`;
        axios.get(url)
            .then(res => {
                const data: CommitApiResponse = res.data;
                if (data.code === 200 && data.data) {
                    setCommits(data.data.commits);
                    setTotal(data.data.total || 0);
                    setError(null);
                } else {
                    setError(data.message || 'Failed to load commits');
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
        fetchCommits();
    }, [owner, repo, page, limit]);

    const handlePageChange = (newPage: number) => {
        setPage(newPage);
    };

    if (loading) {
        return (
            <div className="p-4">
                <div className="bg-white rounded-lg shadow p-6 flex">
                    <Loader2 size={48} className="mr-2 text-blue-500" />
                    <span className="mt-4 text-lg">Loading commits...</span>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="p-4">
                <div className="bg-white rounded-lg shadow p-6 flex">
                    <AlertCircle size={48} className="mr-2 text-red-500" />
                    <span className="mt-4 text-lg">{error}</span>
                </div>
            </div>
        );
    }

    return (
        <div className="p-4">
            {Array.isArray(commits) && commits.length ? (
                <div style={{ height: 'calc(100vh - 250px)' }} className="overflow-y-auto">
                    <div>
                        {commits.map((commit, index) => (
                            <div key={index} className="bg-white rounded-lg shadow mb-2 p-3">
                                <span className="font-bold">{commit.message}</span>
                                <div className="flex justify-between mt-2">
                                    <span className="text-sm text-gray-500">
                                        {commit.author} · {new Date(commit.timestamp * 1000).toLocaleString()}
                                    </span>
                                    <span className="text-sm italic text-gray-500">
                                        {commit.hash.slice(0,7)}
                                    </span>
                                </div>
                            </div>
                        ))}
                    </div>
                    <Group justify="center" mt={4}>
                        <Button
                            disabled={page === 1}
                            onClick={() => handlePageChange(Math.max(1, page - 1))}
                        >
                            Previous
                        </Button>
                        <Text mx={2}>Page {page}</Text>
                        <Button
                            disabled={commits.length < limit || page * limit >= total}
                            onClick={() => handlePageChange(page + 1)}
                        >
                            Next
                        </Button>
                    </Group>
                </div>
            ) : (
                <div className="p-6 flex flex-col items-center justify-center">
                    <span>No commits found</span>
                </div>
            )}
        </div>
    );
};