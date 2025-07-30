import {Input, Pagination} from "@mantine/core";
import {useEffect, useState} from "react";
import axios from "axios";
import {useNavigate} from "react-router-dom";
export interface RepoListTypes {
    name: string,
    description: string,
    owner: string,
    owner_name: string,
    created_at: Date,
    updated_at: Date,
    uid: string,
}
const Limit = 20;

export const RootRepoList = () => {
    const [Page,setPage] = useState(1);
    const [RepoList,setRepoList] = useState<RepoListTypes []>([]);
    const [Total, setTotal] = useState(0);
    const [Sort, _setSort] = useState<"name_asc" | "name_desc" | "created_asc" | "created_desc" | "updated_asc" | "updated_desc" >("updated_asc");
    const [Search, setSearch] = useState("");
    const FetchRepoList = () => {
        axios.get("/api/repo/list", {
            params: {
                page: Page - 1,
                limit: Limit,
                order: Sort,
                name: Search,
            }
        })
        .then((res) => {
            setRepoList(res.data.data.list);
            setTotal(res.data.data.total)
        })
    }
    useEffect(() => {
        FetchRepoList();
    }, [Page, Limit, Sort, Search]);
    useEffect(() => {
        FetchRepoList();
    }, []);
    const nav = useNavigate();
    return(
        <div className="root">
            <div className="root-header">
                <Input
                    placeholder="Search for a repository..."
                    onChange={(e) => {
                        setSearch(e.target.value);
                    }}
                    style={{
                        width: "100%",
                        marginBottom: "1rem",
                        marginTop: "1rem",
                        marginLeft: "1rem",
                        marginRight: "1rem",
                        borderRadius: "5px",
                    }}
                />
            </div>
            <div className="root-body">
                {
                    RepoList.map((repo, index) => {
                        return (
                            <div
                                onClick={()=>{
                                    nav(`/${repo.owner_name}/${repo.name}`)
                                }}
                                key={index}
                                style={{
                                    padding: '1rem',
                                    borderBottom: '1px solid #eee',
                                    backgroundColor: '#fff',
                                    transition: 'background-color 0.2s',
                                    borderRadius: '20',
                                    cursor: 'pointer',
                                }}
                            >
                                <div style={{
                                    fontSize: '1.2rem',
                                    fontWeight: 'bold',
                                    color: '#333',
                                    marginBottom: '0.5rem'
                                }}>
                                    {repo.owner_name} / {repo.name}
                                </div>
                                <div style={{
                                    color: '#666',
                                    fontSize: '0.9rem',
                                    marginBottom: '0.5rem'
                                }}>
                                    {repo.description || 'No description yet'}
                                </div>
                                <div style={{
                                    display: 'flex',
                                    justifyContent: 'space-between',
                                    fontSize: '0.8rem',
                                    color: '#999'
                                }}>
                    <span>
                        Created: {new Date(repo.created_at).toString()}
                    </span>
                    <span>
                        Updated: {new Date(repo.updated_at).toString()}
                    </span>
                                </div>
                            </div>
                        )
                    })
                }
            </div>
            <Pagination total={Total} value={Page} onChange={setPage}/>
        </div>
    )
}