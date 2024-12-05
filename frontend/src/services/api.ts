import axios from "axios";

const api = axios.create({
    baseURL: "http://127.0.0.1:8080",
    timeout: 2000,
    headers: {
        "Content-Type": "application/json",
    }
});

export const fetchVotes = async () => {
    const response = await api.get("/votes");
    return response.data;
}

export const fetchVoteById = async (id: number) => {
    const response = await api.get(`/votes/${id}`);
    return response.data;
}

export const fetchAddVote = async (title: string, description: string) => {
    try {
        const response = await api.post(`/vote`, {
            title: title, 
            description: description,
        });
        return response.data;
    } catch (error) {
        console.log('Ошибка при выполнении запроса: ', error);
    }
}

export default api;