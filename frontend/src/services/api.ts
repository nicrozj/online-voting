import axios from "axios";

const api = axios.create({
    baseURL: "http://127.0.0.1:8080",
    timeout: 2000,
    headers: {
        "Content-Type": "application/json",
    }
});

export const getVotes = async () => {
    try {
        const response = await api.get("/votes");
        return response.data;
    }
    catch (err) {
        console.log(`Ошибка при изъятии голосований: ${err}`);
    }
}

export const getVoteById = async (id: number) => {
    try {
        const response = await api.get(`/vote/${id}`);
        return response.data;
    }
    catch (err) {
        console.log(`Ошибка при изъятии голосования: ${err}`);
    }
}

export const deleteVoteById = async (id: number) => {
    try {
        const response = await api.delete(`/vote/${id}`);
        return response.data;
    }
    catch (err) {
        console.log(`Ошибка при удалении: ${err}`);
    }
}

export const addVote = async (title: string, description: string, options: Array<string>) => {
    let id: number;
    try {
        const response = await api.post(`/vote`, {
            title: title, 
            description: description,
        });
        id = response.data;
    } catch (error) {
        console.log('Ошибка при выполнении запроса: ', error);
    }

    try {
        const response = await api.post(`/vote/${id}/options`, { 
            options
        });
        return response.data;
    } catch (error) {
        console.log('Ошибка при выполнении запроса: ', error);
    }
}

export const getOptionsByVoteId = async (id: number) => {
    try {
        const response = await api.get(`/vote/${id}/options`);
        return response.data;
    } catch (error) {
        console.log('Ошибка при выполнении запроса: ', error);
    }
}

export default api;