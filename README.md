# check24-best-combination-submission

## 📀 Submission Video
You can find the video on [YouTube](https://youtu.be/LAXtcj1Vctc).

## 🛠️ How to Run Locally

### 📋 Prerequisites
This installation assumes you are using a MacOS device with [Homebrew](https://brew.sh) already installed. The approach is similar for Linux machines. While the application itself is runnable on Linux, it hasn't been thoroughly tested there.

Additionally, ensure that [Rust](https://www.rust-lang.org/tools/install) is installed for running services locally or performing benchmarks.

### 📦 Installation

Assuming you are using `Homebrew`, install the following dependencies:

```
# Replace `~` with the dependency name
$ brew install ~

# Required
cmake
protobuf
docker-compose
docker

# Optional: depending on your prefered method of setting up the docker deamon
colima
qemu

# Optional: if the frontend isn't reachable via the container
node 
pnpm
```
### 🔧 Set the Environment Variables

Copy `~/.env.example` and `~/frontend/.env.example`, then rename them to `.env` so they can be picked up by the application. Feel free to adjust credentials as needed.

### 🐳 Start Containers
Depending on your preferred method for setting up the Docker daemon, ensure it is running.

I personally use `colima`:
```bash
$ colima start --cpu 4 --memory 8
```
Ensure you have the following set in your `.zshrc` or similar:
```bash
# --- DOCKER AND COLIMA CONFIGURATION ---
# Set the DOCKER_HOST to point to Colima's Docker socket
export DOCKER_HOST=unix://${HOME}/.colima/default/docker.sock
```
Start the containers:
```bash
$ docker-compose up
```
The first build might take some time until completion. Afterward, you can reach the frontend at [http://localhost:5000](http://localhost:5000). Grab a coffee while waiting. ☕️

### ⚠️ Frontend Container Issue 
> [!NOTE]
> Depending on the browser and machine, the frontend might not be reachable. If you encounter issues, start the frontend manually.
 
```bash
$ cd frontend
$ pnpm i
$ pnpm run dev
```
The frontend should now be accessible with `Astro` on [http://localhost:4321](http://localhost:4321).

### 📈 Dashboards
Once you've started the containerized environment, additional dashboards and utilities become available.

#### Apollo Sandbox
You can test the GraphQL endpoints via the Apollo gateway at the Apollo Sandbox: [http://localhost:4000](http://localhost:4000).

Feel free to test the endpoints for the best combinations and to understand the data structures used for the frontend UI.

#### RabbitMQ 
Track the queues, consumers, and messages related to the `api-service` and `worker-services` at [http://localhost:15672](http://localhost:15672).

If not changed via the `.env` the credentials are:
```
username: root
password: example
```

#### Grafana
There are two dashboards built for the `api-service` and `data-fetch-service`, which can be used to monitor request times, errors, and more at [http://localhost:3000](http://localhost:3000).

If not changed via the `.env`, the credentials are:
```
username: root
password: example
```

## 🚀 Approach

### 🛠️ Tech Stack

![tech_stack](assets/tech_stack.png)

**Technologies Used:**
- **Rust:** Why? I love rust. Fast, memory-safe, and enjoyable to work with. 🤠
- **MongoDB:** Chosen for persistence with MongoDB Compass and indexes for convenience and maintainability.
- **Redis:** Used to store best combination results in cache to mitigate the need to enqueue jobs to the worker-services.
- **RabbitMQ:** Delegates best combination workloads to distributed worker nodes that pick jobs from the queue and store their results in cache.
- **Docker:** Orchestration tool for the distributed systems.
- **Apollo:** Combines the services’ GraphQL endpoints into a single API for ease of use by the frontend.
- **GraphQL:** Preferred over REST for dynamic data retrieval tailored to the client's needs.
- **Prometheus:** Introduces metrics for the services, allowing them to be monitored.
- **Grafana:** Visualizes the scraped metrics from Prometheus in dashboards.
- **GitHub (Actions):** Ensures working production code through continuous integration.
- **Node.js:** Utilized for the frontend.
- **Astro:** Preferred for the UI due to its pure HTML and client-side JS, enabling quick and responsive interfaces.
- **React:** Used for building custom components for visualizing the best combinations.
- **Tailwind CSS:** Facilitates the creation of a visually appealing UI with ease.

### 👨🏼‍💻 User Interface

![ui](assets/ui.png)
![bc_card](assets/bc_card.png)

### 🔍 Set Cover Best Combinations Algorithm

#### 📚 Overview
Given the provided dataset for the challenge and the requirement of finding the best combination of packages, this problem can be described as a [Set Cover problem](https://en.wikipedia.org/wiki/Set_cover_problem).
The core of this project implements a recursive approach to the Set Cover problem, enumerating multiple optimal combinations based on a greedy cost-based strategy.
The algorithm identifies all best combinations of streaming package subsets that cover a given universe of game IDs, considering both exact and approximate coverage scenarios.

#### ⁉️ Problem
Using a basic iterative set cover algorithm would yield a solution. However, given the use case and the fact that there's no polynomial-time solution (NP-hardness), a more optimized approach is necessary to handle the specific requirements effectively.

#### 📐 Mathematical Definitions

##### Input

- **Universe**: $\( U = \{g_1, g_2, \dots, g_n\} \)$, a set of game IDs that must be covered.
- **Subsets**: $\( S = \{S_1, S_2, \dots, S_m\} \)$, where each $\( S_i \subseteq U \)$ represents a streaming package, associated with a cost $\( c_i \)$.
- **Limit**: $\( L \)$, the maximum number of best combinations to return.

##### Output

- **Best Combinations**: A set of combinations $\( C = \{C_1, C_2, \dots, C_k\} \)$ where each $\( C_j \subseteq S \)$ satisfies:
  - **Coverage**: $\( \bigcup_{S_i \in C_j} S_i = U \)$
  - **Cost Minimization**: Each $\( C_j \)$ minimizes the total cost $\( \sum_{S_i \in C_j} c_i \)$
  - **Limit Constraint**: $\( k \leq L \)$

##### Pseudocode

```pseudo
function get_best_combinations(U, S, L):
    results = []
    current_cover = []
    enumerate_best_combinations(U, S, L, results, current_cover)
    return results

function enumerate_best_combinations(U, S, L, results, current_cover):
    covered = union of elements in current_cover

    if covered == U or |current_cover| >= |S|:
        result = map_to_best_combination(current_cover)
        if result not in results:
            results.append(result)
            if |results| >= L:
                return true  // Stop recursion
        return false  // Continue searching

    ratios = []
    for i, subset in enumerate(S):
        uncovered = |subset.elements - covered|
        if uncovered > 0:
            cost = subset.cost
            ratio = cost / uncovered
            ratios.append((i, ratio))

    sort ratios by ascending ratio

    for (i, ratio) in ratios:
        current_cover.append(S[i].id)
        if enumerate_best_combinations(U, S, L, results, current_cover):
            return true
        current_cover.pop()
        if |results| >= L:
            return true

    if branch_explored and current_cover not empty:
        result = map_to_best_combination(current_cover)
        if result not in results:
            results.append(result)
    return false
```

#### 🧮 Final Algorithm
I developed an `enumerated greedy recursive set cover algorithm` in code, which I benchmarked initially without additional overhead:
```rust
/// Computes a set of best combinations of streaming package subsets that cover a given universe of game IDs.
///
/// # Overview
///
/// `get_best_combinations` attempts to find one or more best combinations of these packages (subsets)
/// that cover the entire universe of game IDs. A combination is considered covering the universe if
/// every game ID in the universe is included in at least one offer of a chosen package.
///
/// In addition, it can also consider use cases with non-existent set coverage, where it tries to
/// approximate an arbitrary number of solutions, which get as close as possible.
///
/// Under the hood, this method uses a greedy recursive backtracking strategy, guided by heuristics like
/// sorting subsets according to cost or cost-per-uncovered-element ratios. While heuristics and pruning
/// strategies may help in practice, the underlying problem is NP-hard. Thus, this algorithm can still
/// exhibit exponential runtime in the worst case.
///
/// # Example Scenario
///
/// Suppose we have a universe U = {1, 2} and subsets:
/// - S1 covers {1} with cost 5
/// - S2 covers {1} with cost 5
/// - S3 covers {2} with cost 5
///
/// Both (S1, S3) and (S2, S3) form covers of U, making multiple equally viable solutions.
/// The algorithm enumerates these solutions, which can be beneficial if you want a set
/// of candidate solutions for further analysis.
///
/// # Arguments
///
/// * `universe` - A `BTreeSet<usize>` representing all game IDs that must be covered.
/// * `subsets` - A slice of `BestCombinationSubsetDto` representing candidate streaming packages.
/// * `limit` - The maximum number of solutions (combinations of subsets) to return.
///
/// # Returns
///
/// `Vec<BestCombinationDto>`: A vector of best combinations.
///
pub fn get_best_combinations(
    universe: &BTreeSet<usize>,
    subsets: &[BestCombinationSubsetDto],
    limit: usize,
) -> Vec<BestCombinationDto> {
    let mut results: Vec<BestCombinationDto> = Vec::new();
    let mut current_cover: Vec<usize> = Vec::new();
    enumerate_best_combinations(universe, subsets, limit, &mut results, &mut current_cover);
    results
}

/// Recursively enumerates possible combinations of subsets that cover the given universe of game IDs.
///
/// # Overview
///
/// `enumerate_best_combinations` is the core logic behind `get_best_combinations`. Using backtracking,
/// it attempts to build complete solutions by selecting subsets:
///
/// 1. At each recursive call, it evaluates which subsets best improve coverage of the remaining uncovered games.
/// 2. It selects the next best candidate according to the cost per uncovered games
/// 3. If a full cover is found or it reaches a leaf node, the current combination is recorded as a solution.
/// 4. The function then attempts to find more solutions (up to the specified `limit`) by backtracking and trying
///    alternate subsets.
///
/// # Arguments
///
/// * `universe` - The full set of game IDs that must be covered.
/// * `subsets` - The collection of candidate streaming packages (no duplicates assumed).
/// * `limit` - The maximum number of solutions to return. Once reached, the search halts.
/// * `results` - A mutable reference to a vector collecting all found solutions.
/// * `current_cover` - A mutable vector representing the current partial solution (as a list of chosen subset IDs).
///
/// # Returns
///
/// Returns `true` if more solutions can still be found (meaning it will continue searching), or `false`
/// if the limit has been reached or no further solutions are possible.
///
fn enumerate_best_combinations(
    universe: &BTreeSet<usize>,
    subsets: &[BestCombinationSubsetDto],
    limit: usize,
    results: &mut Vec<BestCombinationDto>,
    current_cover: &mut Vec<usize>,
) -> bool {
    let covered: BTreeSet<usize> = current_cover
        .iter()
        .flat_map(|&id| {
            subsets
                .iter()
                .find(|s| s.streaming_package_id == id)
                .unwrap()
                .elements
                .iter()
                .map(|elem| elem.game_id)
        })
        .collect();

    // Check if all elements are covered or if a leaf node has been reached
    if covered == *universe || current_cover.len() >= subsets.len() {
        let result =
            mapper::map_to_best_combination_dto(current_cover, subsets, universe, results.len());
        if !results.iter().any(|r| r.is_duplicate_of(&result)) {
            results.push(result);
            if results.len() >= limit {
                return true; // Signal to stop further recursion
            }
        }
        return false; // Continue searching if limit not reached
    }

    // Calculate cost-benefit ratio for each subset based on uncovered elements
    let mut ratios: Vec<(usize, f64)> = subsets
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            let uncovered_elements = s.element_ids().difference(&covered).count();

            if uncovered_elements > 0 {
                let cost = if CONFIG.use_yearly_price {
                    s.monthly_price_yearly_subscription_in_cents as f64
                } else {
                    // Use a high value if monthly_price_cents is None to effectively exclude this subset
                    s.monthly_price_cents.unwrap_or(usize::MAX) as f64
                };
                Some((i, cost / uncovered_elements as f64))
            } else {
                None // skip subsets that don't add coverage
            }
        })
        .collect();

    // Sort subsets based on ascending ratio (lower is better)
    ratios.sort_by(|(_, ratio1), (_, ratio2)| {
        ratio1
            .partial_cmp(ratio2)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut branch_explored = true;

    for (i, _) in ratios.iter() {
        current_cover.push(subsets[*i].streaming_package_id);

        // Recurse and check if it should step
        if enumerate_best_combinations(universe, subsets, limit, results, current_cover) {
            return true;
        };

        current_cover.pop();

        // If it exits here, the branch has been fully explored
        branch_explored = false;

        // If limit is reached, stop
        if results.len() >= limit {
            return true;
        }
    }

    // If the branch is fully explored, save the current cover as the closest achievable
    if branch_explored && !current_cover.is_empty() {
        let result =
            mapper::map_to_best_combination_dto(current_cover, subsets, universe, results.len());
        if !results.iter().any(|r| r.is_duplicate_of(&result)) {
            results.push(result);
        }
    }

    false // Continue searching
}
```
> [!NOTE]
> You can adjust the ratio behavior via the .env file by setting `USE_YEARLY_PRICE` to true, which configures the algorithm to build ratios based on yearly subscription prices.

### ⚡️ Benchmarking
From the beginning of this project, my primary goal was not only to find a solution for the best combination but also to ensure it was fast and capable of finding alternatives.
I benchmarked my initial solution, which was a basic iterative set cover algorithm, against the final version assuming the **worst-case scenario** of all game IDs:

**Iterative:**
![set_cover_comp_1](assets/set_cover_comp_1.png)

**Recursive:**
![set_cover_comp_2](assets/set_cover_comp_2.png)

The results showed that the final version could find additional valid set covers and set cover approximations while also being faster, achieving **sub-5ms** benchmarks.

Additionally, I evaluated how the algorithm performs when finding subsequent best combinations:

**Final Algorithm:**
![recursive_set_cover](assets/recursive_set_cover.png)
> Input being the number of best combinations to retrieve

The benchmark demonstrated efficient performance when identifying the top 4 best combinations being **~10ms**.

For running the benchmark on your machine:
```bash
$ cd benchmarks
$ cargo bench -v
```

### 🚦 Loadtests
I conducted load tests to assess how the backend performs under high usage:

![loadtest](assets/loadtest.png)

The results indicate that the average response time is processed in less than **50ms**, making it suitable for real-world usage and ensuring a responsive UI.

You can conduct the load test using k6 while viewing the dashboards for monitoring the API:
```rust
# ensure k6 by grafana is installed
$ brew install k6
$ k6 run loadtests/loadtest.js
```

## ⚙️ Optimizations
> [!NOTE]
> Please note that the benchmark and load test data were gathered on an M1 MacBook Pro. Results may vary depending on the system.

During load tests, it was notable that the workers took a significant amount of time to acknowledge messages received via the queue.
This delay is due to the preprocessing of the best combinations subset.
The aggregation pipeline utilizes two nested `$lookup` operations, which introduces substantial overhead, as seen via the RabbitMQ dashboard:

![unoptimized-aggregation](assets/unoptimized-aggregation.png)
> Red: jobs enqueued; Purple: jobs acknowledged

**Dataset:**
![db](assets/db.png)

**Pipeline:**
```
"$lookup": {
    "from": "bc_streaming_offer",
    "localField": "streaming_package_id",
    "foreignField": "streaming_package_id",
    "as": "offers",
    "pipeline": [
        {
            "$match": {
                "game_id": {
                    "$in":  game_ids
                }
            }
        },
        {
            "$lookup": {
                "from": "bc_game",
                "localField": "game_id",
                "foreignField": "game_id",
                "as": "game",
                "pipeline": [
                    {
                        "$project": {
                            "_id": 0,
                            "tournament_name": 1
                        }
                    }
                ]
            }
        },
        {
            "$unwind": "$game"
        },
        {
            "$project": {
                "_id": 0,
                "game_id": 1,
                "tournament_name": "$game.tournament_name",
                "live": 1,
                "highlights": 1,
            }
        }
    ]
}
```

After benchmarking and profiling, this was identified as the only remaining bottleneck.

**Suggested Approaches:**
1. Embed Subset Documents:
	- Embed the subset documents directly to reduce the need for `$lookup` operations.
	- **Pros:** Improved performance by minimizing join operations.
	- **Cons:** Increased memory usage due to data duplication.
2. Denormalization Techniques:
	- Create an additional collection to store pre-joined data.
	- **Pros:** Balances memory usage and performance.
	- **Cons:** Requires maintaining data consistency across collections.

More information on these approaches can be found [here](https://www.mongodb.com/docs/atlas/schema-suggestions/reduce-lookup-operations/?utm_source=compass&utm_medium=product#learn-more).

Given the provided dataset i've stayed away from these 2 approaches, as i wanted to stay true to the original data schema. Thus i've build my api around polling and prefetching. 
As the user enters and removes entries one by one via the UI we can enqueue mutations, which will preload the Cache with results.
Thus when the client want's to actually retrieve the final best combinations, he'll still receive the desired data in less than **50ms** average.

## License
This project is available under the MIT License.
