import {AlertCircle, FileText, GitBranch, Loader2} from "lucide-react";
import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import axios from "axios";
import type {Repository} from "@/app/repository/layout.tsx";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from 'remark-math';
import remarkBreaks from 'remark-breaks';
import remarkEmoji from 'remark-emoji';
import {Prism as SyntaxHighlighter} from "react-syntax-highlighter";
import {MdFileDownload, MdOutlineContentCopy} from "react-icons/md";
import rehypeRaw from "rehype-raw";
import rehypeKatex from "rehype-katex";
interface FileData {
    repo: Repository,
    tree: {
        authors: {
            [authorId: string]: {
                email: string;
                name: string;
                time: number;
            };
        },
        data: CommitData[];
        file: [FileInfo, number][];
    }
}

export interface AuthorInfo {
    email: string;
    name: string;
    time: number;
}

export interface CommitData {
    author: string;
    committer: string;
    index: number;
    message: string;
    oid: string;
    timestamp: number;
}

export interface FileInfo {
    name: string;
    path: string;
    rtype: string;
}
interface FileApiResponse {
    code: number;
    data: FileData;
    message: string;
}

export const RepoFiles = () => {
    const { owner, repo, '*': path } = useParams();
    const navigate = useNavigate();
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [files, setFiles] = useState<[FileInfo, number][]>([]);
    const [commits, setCommits] = useState<CommitData[]>([]);
    const fetchFiles = () => {
        if (!owner || !repo) return;
        setLoading(true);
        const currentPath = path || '';
        const url = `/api/repo/${owner}/${repo}/tree/${currentPath}`;
        axios.get(url)
            .then(res => {
                const data: FileApiResponse = res.data;
                if (data.code === 200 && data.data) {
                    setFiles(data.data.tree.file);
                    setCommits(data.data.tree.data);
                    setError(null);
                } else {
                    setError(data.message || 'Failed to load files');
                }
            })
            .catch(err => {
                setError(err.message || 'Network error occurred');
            })
            .finally(() => {
                setLoading(false);
            });
    };
    const [Readme, setReadme] = useState<string>("")
    useEffect(() => {
        setReadme("")
        Object.values(files)
            .map(([info, _])=>{
                if (info.name === "README.md") {
                    axios.get(`/api/repo/${owner}/${repo}/cat_file/${info.path}${info.name}`)
                        .then(res=>{
                            setReadme(res.data);
                        })
                }
            })
    }, [files]);

    useEffect(() => {
        fetchFiles();
    }, [owner, repo, path]);

    if (loading) {
        return (
            <div className="p-4">
                <div className="bg-white rounded-lg shadow-md">
                    <div className="p-6 flex">
                        <Loader2 size={48} className="mr-2 text-blue-500" />
                        <div className="mt-4 text-lg">Loading files...</div>
                    </div>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="p-4">
                <div className="bg-white rounded-lg shadow-md">
                    <div className="p-6 flex">
                        <AlertCircle size={48} className="mr-2 text-red-500" />
                        <div className="mt-4 text-lg">{error}</div>
                    </div>
                </div>
            </div>
        );
    }
    let index = 0;
    return (
        <div className="p-4">
            {files.length ? (
                <div className="h-auto overflow-auto">
                    <div>
                        {
                            path != "" && path && (
                                <div>
                                    <div className="flex items-center" onClick={() => {
                                        navigate(`/${owner}/${repo}/tree/${path.split('/').slice(0, -1).join('/')}`);
                                    }}>
                                        <GitBranch size={18} className="mr-2 text-blue-500" />
                                        <div>...</div>
                                    </div>
                                </div>
                            )
                        }
                        {files.map(([file, idx], index) => (
                            <div 
                                key={index}
                                className={`cursor-pointer ${index === files.length - 1 ? '' : 'border-b border-gray-200'} flex items-center justify-between`}
                                onClick={() => {
                                    if (file.rtype === 'tree') {
                                        const newPath = path ? `${path}/${file.name}` : file.name;
                                        navigate(`/${owner}/${repo}/tree/${newPath}`);
                                    }
                                }}
                            >
                                <div className="flex items-center">
                                    {file.rtype === 'tree' ? (
                                        <GitBranch size={18} className="mr-2 text-blue-500" />
                                    ) : (
                                        <FileText size={18} className="mr-2" />
                                    )}
                                    <div>{file.name}</div>
                                </div>
                                <div className="flex-1 items-center pl-4">
                                    {commits[idx] && (
                                        <>
                                            {commits[idx].message}
                                        </>
                                    )}
                                </div>
                                <div className="flex items-center">
                                    {commits[idx] && (
                                        <>
                                            {new Date(commits[idx].timestamp * 1000).toString()}
                                        </>
                                    )}
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            ) : (
                <div className="flex justify-center items-center">
                    <div>No files found</div>
                </div>
            )}
            <div className="readme">
                {
                    Readme && Readme !== "" && (
                        <Markdown
                        remarkPlugins={[remarkGfm, remarkMath, remarkBreaks, remarkEmoji]}
                        rehypePlugins={[rehypeKatex, rehypeRaw]}
                        components={{
                            code({children, className}) {
                                const match: any = /language-(\w+)/.exec(className || "");
                                let [isShowCode, setIsShowCode] = useState(true);
                                return (
                                    <>
                                        <div className="code-header">
                                            <div
                                                style={{cursor: "pointer", marginRight: "10px", transformOrigin: "8px"}}
                                                className={isShowCode ? "code-rotate-down" : "code-rotate-right"}
                                                onClick={() => setIsShowCode(!isShowCode)}
                                            >
                                                <MdFileDownload/>
                                            </div>
                                            <div>{match && match[1]}</div>
                                            <div
                                                className="preview-code-copy"
                                            >
                                                {/*{isShowCopy && <span className="opacity-0-1-0 copy-success">Success</span>}*/}
                                                <MdOutlineContentCopy/>
                                            </div>
                                        </div>
                                        {isShowCode && (
                                            <SyntaxHighlighter
                                                showLineNumbers={true}
                                                language={match && match[1]}
                                            >
                                                {String(children).replace(/\n$/, "")}
                                            </SyntaxHighlighter>
                                        )}
                                    </>
                                );
                            },
                            h1({children}) {
                                return (
                                    <h1 id={"heading-" + ++index} className="heading">
                                        {children}
                                    </h1>
                                );
                            },
                            h2({children}) {
                                return (
                                    <h2 id={"heading-" + ++index} className="heading">
                                        {children}
                                    </h2>
                                );
                            },
                            h3({children}) {
                                return (
                                    <h3 id={"heading-" + ++index} className="heading">
                                        {children}
                                    </h3>
                                );
                            },
                            h4({children}) {
                                return (
                                    <h4 id={"heading-" + ++index} className="heading">
                                        {children}
                                    </h4>
                                );
                            },
                            h5({children}) {
                                return (
                                    <h5 id={"heading-" + ++index} className="heading">
                                        {children}
                                    </h5>
                                );
                            },
                            h6({children}) {
                                return (
                                    <h6 id={"heading-" + ++index} className="heading">
                                        {children}
                                    </h6>
                                );
                            },
                        }}
                    >
                        {Readme}
                    </Markdown>
                    )
                }
            </div>
        </div>
    );
};