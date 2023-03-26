use super::*;

pub enum Task {
    MakeSimulations(SimulationTask), 
    HandleResults(TrialResult),

}

impl Task {
    async fn begin (&self) {
        match &self {
            &Self::MakeSimulations(simulation) => {
                simulation.begin();
            }
            &Self::HandleResults(result) => {

            }
        }
    }
}

pub enum SimulationTask {
    DefaultSimulation(Sender<TrialResult>, Trial),
    TimelineSimulation(Sender<TrialResult>, Trial),
}

impl SimulationTask {
    fn begin (&self) {
        match &self {
            &Self::DefaultSimulation(tx, trial) => {
                trial.simulate(*tx);
            }
            &Self::TimelineSimulation(tx, trial) => {
                trial.simulate_with_timeline(*tx);
            }
        }
    }
}

pub enum HandlerTasks 