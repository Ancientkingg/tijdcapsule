<script>
    import './app.css';
    import { Router, Route } from 'svelte-routing';
    import Home from './routes/Home.svelte';
    import Capsule from './routes/Capsule.svelte';
    export const url = '';
</script>

<!-- App.svelte -->

<svelte:head>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
    <link href="https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap" rel="stylesheet">
</svelte:head>

<Router {url}>
    <div>
        <Route path="/capsule/:id" component={Capsule}></Route>
        <Route path="/"><Home></Home></Route>
    </div>
</Router>

<style>
    div {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 100%;
    }
</style>
