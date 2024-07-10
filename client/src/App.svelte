<script>
    import './app.css';
    import { Router, Route } from 'svelte-routing';
    import Home from './routes/Home.svelte';
    import Capsule from './routes/Capsule.svelte';
    export const url = '';
</script>

<!-- App.svelte -->

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
