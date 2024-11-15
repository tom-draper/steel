<script lang="ts">
    import { onDestroy } from "svelte";
    import * as d3 from "d3";
    import { page } from "$app/stores";

    export let files;

    $: if (files) {
        buildPlot();
    }

    let svg;

    // Set chart dimensions
    // const width = 1920;
    // const height = 1080;
    const width = 1920;
    const height = 1080;

    function buildPlot() {
        // Clear any existing SVG content in #graph
        d3.select("#graph").select("svg").remove();

        const root = d3.hierarchy(trimPath(buildHierarchy(files)));

        const links = root.links();
        const nodes = root.descendants();

        // Initialize force simulation
        const simulation = d3
            .forceSimulation(nodes)
            .force(
                "link",
                d3
                    .forceLink(links)
                    .id((d) => d.id)
                    .distance(50)
                    .strength(0.5),
            )
            .force("charge", d3.forceManyBody().strength(-100))
            .force("x", d3.forceX())
            .force("y", d3.forceY())
            .force("center", d3.forceCenter(width / 2 + 50, height / 2 + 50))
            .on("tick", ticked);

        // // Create SVG element
        svg = d3
            .select("#graph")
            .append("svg")
            .attr("width", width)
            .attr("height", height)
            .attr("viewBox", [0, 0, width, height])
            .attr("style", "max-width: 100%;");

        // Append links.
        const link = svg
            .append("g")
            .attr("stroke", "#999")
            .attr("stroke-opacity", 0.6)
            .selectAll("line")
            .data(links)
            .join("line");

        // Append nodes.
        const node = svg
            .append("g")
            .attr("fill", "#fff")
            .attr("stroke", "#000")
            .attr("stroke-width", 1.5)
            .selectAll("circle")
            .data(nodes)
            .join("circle")
            .attr("fill", (d) =>
                d.data.id === $page.params.slug ||
                (d.data.id === "/" && !$page.params.slug)
                    ? "red"
                    : d.children
                      ? null
                      : "#000",
            )
            .attr("stroke", (d) =>
                d.data.id === $page.params.slug ||
                (d.data.id === "/" && !$page.params.slug)
                    ? "red"
                    : d.children
                      ? null
                      : "#fff",
            )
            .attr("r", (d) =>
                d.data.id == $page.params.slug ||
                (d.data.id === "/" && !$page.params.slug)
                    ? 5
                    : 3.5,
            )
            .call(drag(simulation));

        node.append("title").text((d) => d.data.id);

        // Tick function to update positions
        function ticked() {
            link.attr("x1", (d) => d.source.x)
                .attr("y1", (d) => d.source.y)
                .attr("x2", (d) => d.target.x)
                .attr("y2", (d) => d.target.y);

            node.attr("cx", (d) => d.x).attr("cy", (d) => d.y);
        }

        // Drag functions
        function drag(simulation) {
            function dragstarted(event) {
                if (!event.active) simulation.alphaTarget(0.3).restart();
                event.subject.fx = event.subject.x;
                event.subject.fy = event.subject.y;
            }

            function dragged(event) {
                event.subject.fx = event.x;
                event.subject.fy = event.y;
            }

            function dragended(event) {
                if (!event.active) simulation.alphaTarget(0);
                event.subject.fx = null;
                event.subject.fy = null;
            }

            return d3
                .drag()
                .on("start", dragstarted)
                .on("drag", dragged)
                .on("end", dragended);
        }

        onDestroy(() => {
            // Cleanup the SVG and stop the simulation on component unmount
            simulation.stop();
            svg.remove();
        });
    }

    function generateNodesAndLinks(paths) {
        const nodes = new Map();
        const links = [];

        // Step 1: Create a node for each unique directory and file
        paths.forEach((path) => {
            const parts = path.split("/");

            // Build up the path part by part
            parts.reduce((parentPath, part) => {
                const currentPath = parentPath ? `${parentPath}/${part}` : part;

                // Add the current path as a node if it doesn’t already exist
                if (!nodes.has(currentPath)) {
                    nodes.set(currentPath, {
                        id: currentPath,
                        group: parts.length,
                    });
                }

                // If there’s a parent path, create a link to the current path
                if (parentPath) {
                    links.push({ source: parentPath, target: currentPath });
                }

                return currentPath; // The current path becomes the new parentPath for the next iteration
            }, "");
        });

        // Step 2: Convert nodes from Map to array format expected by D3
        const nodesArray = Array.from(nodes.values());

        return { nodes: nodesArray, links };
    }

    function buildHierarchy(paths) {
        const root = { name: "root", id: "/", children: [] };

        paths.forEach((path) => {
            const parts = path.split("/");
            let current = root;
            let currentPath = "";

            parts.forEach((part) => {
                currentPath = currentPath ? `${currentPath}/${part}` : part;

                // Find or create the child node
                let child = current.children.find(
                    (child) => child.name === part,
                );
                if (!child) {
                    child = { name: part, id: currentPath, children: [] };
                    current.children.push(child);
                }
                // Move to the child for the next iteration
                current = child;
            });
        });

        return root;
    }

    function trimPath(root) {
        if (!$page.params.slug) {
            return root;
        }

        const parentDirectory = $page.params.slug.split("/").slice(0, -1);

        let _root = root;
        while (_root.children.length === 1 && _root.id !== parentDirectory) {
            _root = _root.children[0];
        }
        return _root;
    }

    // function buildHierarchy(paths) {
    //     const root = { name: "root", id: "/", children: [] };

    //     // Get an array of parent directories to exclude
    //     const parentDirectories = $page.params.slug ? $page.params.slug.split("/").slice(0, -1) : '/';

    //     paths.forEach((path) => {
    //         const parts = path.split("/");
    //         let current = root;
    //         let currentAccumulatedPath = "";

    //         // Skip parent directories by checking against currentPath
    //         let skip = true;

    //         parts.forEach((part, index) => {
    //             currentAccumulatedPath = currentAccumulatedPath
    //                 ? `${currentAccumulatedPath}/${part}`
    //                 : part;

    //             // Skip nodes that are part of parent directories
    //             if (skip) {
    //                 if (
    //                     currentAccumulatedPath === parentDirectories.join("/")
    //                 ) {
    //                     skip = false; // Start including nodes after parent directories
    //                 }
    //                 return;
    //             }

    //             // Find or create the child node
    //             let child = current.children.find(
    //                 (child) => child.name === part,
    //             );
    //             if (!child) {
    //                 child = {
    //                     name: part,
    //                     id: currentAccumulatedPath,
    //                     children: [],
    //                 };
    //                 current.children.push(child);
    //             }
    //             // Move to the child for the next iteration
    //             current = child;
    //         });
    //     });

    //     return root;
    // }


</script>

<div class="fixed top-0 left-0 w-full grid place-items-center z-10">
    <div id="graph"></div>
</div>
