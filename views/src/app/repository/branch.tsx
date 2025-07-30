import {AlertCircle, GitBranch, Loader2} from "lucide-react";
import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import axios from "axios";

interface Branch {
    default: boolean;
    head: string;
    name: string;
    time: string;
}

interface BranchApiResponse {
    code: number;
    data: Branch[];
    message: string;
}

export const RepoBranch = () => {
    const { owner, repo } = useParams<{ owner: string; repo: string }>();
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [branches, setBranches] = useState<Branch[]>([]);

    const fetchBranches = () => {
        if (!owner || !repo) return;

        setLoading(true);
        const url = `/api/repo/${owner}/${repo}/branches`;
        axios.get(url)
            .then(res => {
                const data: BranchApiResponse = res.data;
                if (data.code === 200 && data.data) {
                    setBranches(data.data);
                    setError(null);
                } else {
                    setError(data.message || 'Failed to load branches');
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
        fetchBranches();
    }, [owner, repo]);

    if (loading) {
        return (
            <div className="p-4">
                <div className="bg-white rounded-lg shadow p-6 flex">
                    <Loader2 size={48} className="mr-2 text-blue-500" />
                    <div className="mt-4 text-lg">Loading branches...</div>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="p-4">
                <div className="bg-white rounded-lg shadow p-6 flex">
                    <AlertCircle size={48} className="mr-2 text-red-500" />
                    <div className="mt-4 text-lg">{error}</div>
                </div>
            </div>
        );
    }

    return (
        <div className="p-4">
            {branches.length ? (
                <div style={{ height: 'calc(100vh - 250px)' }} className="overflow-y-auto">
                    <div>
                        {branches.map((branch, index) => (
                            <div key={index} className="p-2 border-b border-gray-200 flex items-center justify-between">
                                <div className="flex items-center">
                                    <GitBranch size={18} className="mr-2 text-{branch.default ? 'green-500' : 'blue-500'}" />
                                    <div>{branch.name}</div>
                                    {branch.default && (
                                        <div className="ml-2 text-sm text-green-500">(Default)</div>
                                    )}
                                </div>
                                <div className="text-sm text-gray-500">{branch.head.slice(0, 7)}</div>
                            </div>
                        ))}
                    </div>
                </div>
            ) : (
                <div className="p-6 flex flex-col items-center justify-center">
                    <div>No branches found</div>
                </div>
            )}
        </div>
    );
};