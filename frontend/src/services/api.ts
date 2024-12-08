import axios from "axios";

const api = axios.create({
    baseURL: "http://127.0.0.1:8080",
    timeout: 2000,
    headers: {
        "Content-Type": "application/json",
    }
});

export const getPollings = async () => {
    try {
        const response = await api.get("/pollings");
        return response.data;
    }
    catch (err) {
        console.log(`Ошибка при изъятии голосований: ${err}`);
    }
}

export const getPollingById = async (id: number) => {
    try {
        const response = await api.get(`/pollings/${id}`);
        return response.data;
    }
    catch (err) {
        console.log(`Ошибка при изъятии голосования: ${err}`);
    }
}

export const deletePollingById = async (id: number) => {
    try {
        const response = await api.delete(`/pollings/${id}`);
        return response.data;
    }
    catch (err) {
        console.log(`Ошибка при удалении: ${err}`);
    }
}

export const addPolling = async (title: string, description: string, options: Array<string>) => {
    let id: number;
    try {
        const response = await api.post(`/pollings`, {
            title: title, 
            description: description,
        });
        id = response.data;
    } catch (error) {
        console.log('Ошибка при выполнении запроса: ', error);
    }

    try {
        const response = await api.post(`/pollings/${id}/options`, { 
            options
        });
        return response.data;
    } catch (error) {
        console.log('Ошибка при выполнении запроса: ', error);
    }
}

export const getOptionsByPollingId = async (id: number) => {
    try {
        const response = await api.get(`/pollings/${id}/options`);
        return response.data;
    } catch (error) {
        console.log('Ошибка при выполнении запроса: ', error);
    }
}

export default api;